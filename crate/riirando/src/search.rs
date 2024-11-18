use {
    std::{
        collections::{
            HashMap,
            HashSet,
        },
        ops::Not,
    },
    collect_mac::collect,
    enum_iterator::{
        Sequence,
        all,
    },
    enumset::{
        EnumSet,
        enum_set,
    },
    itertools::Itertools as _,
    petgraph::matrix_graph::DiMatrix,
    riirando_common::*,
    crate::logic::{
        AnonymousEvent,
        NamedEvent,
        Region,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence)]
pub(crate) enum Age {
    Child,
    Adult,
}

impl Not for Age {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Self::Child => Self::Adult,
            Self::Adult => Self::Child,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence)]
pub(crate) enum TimeOfDay {
    Noon,
    Dampe,
    Midnight,
}

impl TimeOfDay {
    pub(crate) fn is_day(&self) -> bool {
        match self {
            Self::Noon => true,
            Self::Dampe => false,
            Self::Midnight => false,
        }
    }

    pub(crate) fn is_night(&self) -> bool { !self.is_day() }
}

/// World state that changes over a seed and is reversible but persists across savewarps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence)]
pub(crate) struct GlobalState {
    pub(crate) age: Age,
    pub(crate) time_of_day: TimeOfDay,
    pub(crate) savewarp: Savewarp,
    //TODO health, FW placement?
}

pub(crate) trait InventoryContents {
    fn is_in(self, inventory: &Inventory) -> bool;
}

impl InventoryContents for Item {
    fn is_in(self, inventory: &Inventory) -> bool {
        inventory.base.contains(self)
    }
}

impl InventoryContents for EnumSet<Item> {
    fn is_in(self, inventory: &Inventory) -> bool {
        inventory.base.is_superset(self)
    }
}

impl InventoryContents for (Item, usize) {
    fn is_in(self, inventory: &Inventory) -> bool {
        let (item, required_count) = self;
        match required_count {
            0 => true,
            1 => inventory.base.contains(item),
            _ => inventory.multiples.get(&item).is_some_and(|&found_count| found_count >= required_count),
        }
    }
}

impl InventoryContents for AnonymousEvent {
    fn is_in(self, inventory: &Inventory) -> bool {
        inventory.anonymous_events.contains(&self)
    }
}

impl InventoryContents for NamedEvent {
    fn is_in(self, inventory: &Inventory) -> bool {
        inventory.named_events.contains(self)
    }
}

pub(crate) struct Inventory {
    base: EnumSet<Item>,
    multiples: HashMap<Item, usize>,
    anonymous_events: HashSet<AnonymousEvent>,
    named_events: EnumSet<NamedEvent>,
}

impl Inventory {
    fn starting() -> Self {
        Self {
            base: enum_set!(Item::Wallet),
            multiples: HashMap::default(),
            anonymous_events: HashSet::default(),
            named_events: EnumSet::default(),
        }
    }

    pub(crate) fn contains(&self, item: impl InventoryContents) -> bool {
        item.is_in(self)
    }

    fn collect(&mut self, item: Item) {
        if !self.base.insert(item) {
            //TODO only track multiples where relevant
            *self.multiples.entry(item).or_insert(1) += 1;
        }
    }
}

fn max_explore(region_access: &mut [HashMap<Region, HashSet<GlobalState>>], inventory: &mut Inventory) {
    loop {
        let mut progress_made = false;
        for world_region_access in &mut *region_access {
            for (region, states) in world_region_access.clone() {
                let info = region.info();
                for (item, accesses) in info.items {
                    if !inventory.contains(item) /*TODO track by locations to allow collecting multiples */ && states.iter().any(|state| accesses.iter().any(|access| access(state, inventory))) { //TODO track multiple instances of items when relevant
                        inventory.collect(item);
                        progress_made = true;
                    }
                }
                for (vanilla_target, access) in info.exits {
                    for state in &states {
                        if !world_region_access.get(&vanilla_target).is_some_and(|already_reachable_states| already_reachable_states.contains(state)) && access(state, inventory) {
                            let target_info = vanilla_target.info();
                            match target_info.time_of_day {
                                TimeOfDayBehavior::None => {
                                    world_region_access.entry(vanilla_target).or_default().insert(*state);
                                    world_region_access.entry(Region::Root).or_default().insert(GlobalState { savewarp: target_info.savewarp, ..*state });
                                    if let Region::BeyondDoorOfTime = vanilla_target {
                                        // can time travel here
                                        let age_change = GlobalState { age: !state.age, ..*state };
                                        world_region_access.entry(vanilla_target).or_default().insert(age_change);
                                        world_region_access.entry(Region::Root).or_default().insert(GlobalState { savewarp: target_info.savewarp, ..age_change });
                                    }
                                }
                                TimeOfDayBehavior::Static => {
                                    //TODO allow setting time to noon or midnight using Sun's Song
                                    world_region_access.entry(vanilla_target).or_default().insert(*state);
                                    world_region_access.entry(Region::Root).or_default().insert(GlobalState { savewarp: target_info.savewarp, ..*state });
                                }
                                TimeOfDayBehavior::Passes => for time_of_day in all() {
                                    world_region_access.entry(vanilla_target).or_default().insert(GlobalState { time_of_day, ..*state });
                                    world_region_access.entry(Region::Root).or_default().insert(GlobalState { savewarp: target_info.savewarp, time_of_day, ..*state });
                                },
                                TimeOfDayBehavior::OutsideGanonsCastle => {
                                    // Time of day outside Ganon's Castle is always Dampé time, but we mark all times of day to avoid an infinite loop from a discrepancy with the check for existing access above.
                                    // Exits from Ganon's Castle all check for Dampé time to avoid this hack from leaking time of day into the rest of the world.
                                    for time_of_day in all() {
                                        world_region_access.entry(vanilla_target).or_default().insert(GlobalState { time_of_day, ..*state });
                                    }
                                    world_region_access.entry(Region::Root).or_default().insert(GlobalState { savewarp: target_info.savewarp, time_of_day: TimeOfDay::Dampe, ..*state });
                                }
                            }
                            progress_made = true;
                        }
                    }
                }
            }
        }
        if !progress_made { break }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("at least one world has no access to Hyrule Field as child, which is required to collect Zelda's Lullaby, which is required to beat the Shadow temple")]
    ChildHyruleFieldAccess(HashMap<Region, HashSet<GlobalState>>),
    #[error("at least one world has no access to Ganondorf's boss room as adult")]
    AdultGanondorfBossRoomAccess,
}

/// Returns an error if the reachability requirements as defined in the settings aren't met.
pub(crate) fn check_reachability(worlds: &[()]) -> Result<(), Error> {
    // We only consider global states in logic if they're reachable from all other global states.
    // This way, even if a player reaches a global state out of logic, they can't get stuck.
    // To avoid a combinatorial explosion, we require each world to do so without outside help.
    let reachable_states = worlds.into_iter().map(|()| {
        let mut reachability_graph = DiMatrix::<_, _>::with_capacity(GlobalState::CARDINALITY);
        let node_indices = all::<GlobalState>().map(|state| reachability_graph.add_node(state)).collect_vec();
        let mut dfs_space = petgraph::algo::DfsSpace::new(&reachability_graph);
        for (from_idx, from) in all::<GlobalState>().enumerate() {
            let mut assumed_access = HashMap::default();
            for (to_idx, to) in all().enumerate() {
                if petgraph::algo::has_path_connecting(&reachability_graph, node_indices[from_idx], node_indices[to_idx], Some(&mut dfs_space)) {
                    // already known to be transitively reachable, don't need to check for direct reachability
                    continue
                }
                // check whether the target state is reachable from the source state
                if assumed_access.is_empty() {
                    assumed_access.insert(Region::Root, collect![from]);
                    max_explore(std::slice::from_mut(&mut assumed_access), &mut Inventory::starting());
                }
                if assumed_access.get(&Region::Root).is_some_and(|states| states.contains(&to)) {
                    reachability_graph.add_edge(node_indices[from_idx], node_indices[to_idx], ());
                    dfs_space = petgraph::algo::DfsSpace::new(&reachability_graph);
                }
            }
        }
        all::<GlobalState>()
            .enumerate()
            .filter(|&(to_idx, _)| node_indices.iter().all(|&from_idx| petgraph::algo::has_path_connecting(&reachability_graph, from_idx, node_indices[to_idx], Some(&mut dfs_space))))
            .map(|(_, to)| to)
            .collect::<HashSet<_>>()
    });
    // The root region is reachable as all states which were proven reachable above.
    let mut region_access = reachable_states
        .map(|world_reachable_states| collect![as HashMap<_, _>: Region::Root => world_reachable_states])
        .collect_vec();
    // Now we start the real search.
    max_explore(&mut region_access, &mut Inventory::starting() /*TODO keep this parameter to check for items required to beat the game */);
    // Search completed, check if we can beat the game.
    for world_region_access in region_access {
        //TODO different win conditions, e.g. ALR, no logic, Triforce Hunt, Bingo
        // needs to be child to collect Zelda's Lullaby, which is required to beat the Shadow temple
        if !world_region_access.get(&Region::HyruleField).is_some_and(|states| states.iter().any(|state| state.age == Age::Child)) {
            return Err(Error::ChildHyruleFieldAccess(world_region_access))
        }
        // needs to be able to reach Ganon
        if !world_region_access.get(&Region::GanondorfBossRoom).is_some_and(|states| states.iter().any(|state| state.age == Age::Adult)) { //TODO check for items required to defeat Ganon (including sword, in preparation for Master Sword shuffle)
            return Err(Error::AdultGanondorfBossRoomAccess)
        }
    }
    Ok(())
}
