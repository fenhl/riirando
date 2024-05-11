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
    itertools::Itertools as _,
    petgraph::matrix_graph::DiMatrix,
    crate::logic::{
        Region,
        TimeOfDayBehavior,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Sequence)]
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Sequence)]
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
}

/// World state that changes over a seed and is reversible but persists across savewarps.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Sequence)]
pub(crate) struct GlobalState {
    pub(crate) age: Age,
    pub(crate) time_of_day: TimeOfDay,
    //TODO spawn position, health
}

fn max_explore(region_access: &mut [HashMap<Region, HashSet<GlobalState>>]) {
    loop {
        let mut progress_made = false;
        for world_region_access in &mut *region_access {
            for (region, states) in world_region_access.clone() {
                for (vanilla_target, access) in region.info().exits {
                    for state in &states {
                        if !world_region_access.get(&vanilla_target).is_some_and(|already_reachable_states| already_reachable_states.contains(state)) && access(state) {
                            match vanilla_target.info().time_of_day {
                                TimeOfDayBehavior::None => {
                                    world_region_access.entry(vanilla_target).or_default().insert(*state);
                                    if let Region::BeyondDoorOfTime = vanilla_target {
                                        // can time travel here
                                        let age_change = GlobalState { age: !state.age, ..*state };
                                        world_region_access.entry(vanilla_target).or_default().insert(age_change);
                                        world_region_access.entry(Region::Root).or_default().insert(age_change); // savewarp after age change
                                    }
                                }
                                TimeOfDayBehavior::Static => { world_region_access.entry(vanilla_target).or_default().insert(*state); } //TODO allow setting time to noon or midnight using Sun's Song
                                TimeOfDayBehavior::Passes => for time_of_day in all() {
                                    world_region_access.entry(vanilla_target).or_default().insert(GlobalState { time_of_day, ..*state });
                                    world_region_access.entry(Region::Root).or_default().insert(GlobalState { time_of_day, ..*state }); // savewarp after waiting for time of day
                                },
                                TimeOfDayBehavior::OutsideGanonsCastle => {
                                    // Time of day outside Ganon's Castle is always Dampé time, but we mark all times of day to avoid an infinite loop from a discrepancy with the check for existing access above.
                                    // Exits from Ganon's Castle all check for Dampé time to avoid this hack from leaking time of day into the rest of the world.
                                    for time_of_day in all() {
                                        world_region_access.entry(vanilla_target).or_default().insert(GlobalState { time_of_day, ..*state });
                                    }
                                    world_region_access.entry(Region::Root).or_default().insert(GlobalState { time_of_day: TimeOfDay::Dampe, ..*state }); // savewarp after setting time of day
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

pub(crate) fn can_win(worlds: &[()]) -> bool {
    // We only consider global states in logic if they're reachable from all other global states.
    // This way, even if a player reaches a global state out of logic, they can't get stuck.
    // To avoid a combinatorial explosion, we require each world to do so without outside help.
    let reachable_states = worlds.into_iter().map(|()| {
        let mut reachability_graph = DiMatrix::<_, _>::with_capacity(GlobalState::CARDINALITY);
        let node_indices = all::<GlobalState>().map(|state| reachability_graph.add_node(state)).collect_vec();
        for (from_idx, from) in all::<GlobalState>().enumerate() {
            let mut assumed_access = HashMap::default();
            for (to_idx, to) in all().enumerate() {
                if petgraph::algo::has_path_connecting(&reachability_graph, node_indices[from_idx], node_indices[to_idx], None) {
                    // already known to be transitively reachable, don't need to check for direct reachability
                    continue
                }
                // check whether the target state is reachable from the source state
                if assumed_access.is_empty() {
                    assumed_access.insert(Region::Root, collect![from]);
                    max_explore(std::slice::from_mut(&mut assumed_access));
                }
                if assumed_access.get(&Region::Root).is_some_and(|states| states.contains(&to)) {
                    reachability_graph.add_edge(node_indices[from_idx], node_indices[to_idx], ());
                }
            }
        }
        let mut dfs_space = petgraph::algo::DfsSpace::new(&reachability_graph);
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
    max_explore(&mut region_access);
    // Search completed, check if we can beat the game.
    region_access.into_iter().all(|world_region_access| {
        // needs to be child to collect Zelda's Lullaby, which is required to beat the Shadow temple
        world_region_access.get(&Region::Root).is_some_and(|states| states.iter().any(|state| state.age == Age::Child))
        // needs to be able to reach Ganon
        && world_region_access.get(&Region::GanondorfBossRoom).is_some_and(|states| states.iter().any(|state| state.age == Age::Adult)) //TODO check for items required to defeat Ganon (including sword, in preparation for Master Sword shuffle)
    }) //TODO different win conditions, e.g. ALR, no logic, Triforce Hunt, Bingo
}