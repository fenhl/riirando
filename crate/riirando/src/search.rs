use {
    std::collections::{
        HashMap,
        HashSet,
    },
    collect_mac::collect,
    enum_iterator::{
        Sequence,
        all,
    },
    itertools::Itertools as _,
    petgraph::matrix_graph::DiMatrix,
    crate::logic::Region,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Sequence)]
pub(crate) enum Age {
    Child,
    Adult,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Sequence)]
enum TimeOfDay {
    Noon,
    Dampe,
    Midnight,
}

/// World state that changes over a seed and is reversible but persists across savewarps.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Sequence)]
pub(crate) struct GlobalState {
    pub(crate) age: Age,
    time_of_day: TimeOfDay,
    //TODO spawn position, health
}

pub(crate) fn can_win(worlds: &[()]) -> bool {
    // We only consider global states in logic if they're reachable from all other global states.
    // This way, even if a player reaches a global state out of logic, they can't get stuck.
    // To avoid a combinatorial explosion, we require each world to do so without outside help.
    let reachable_states = worlds.into_iter().map(|()| {
        let mut reachability_graph = DiMatrix::<_, _>::with_capacity(GlobalState::CARDINALITY);
        let node_indices = all::<GlobalState>().map(|state| reachability_graph.add_node(state)).collect_vec();
        for (from_idx, from) in all().enumerate() {
            for (to_idx, to) in all().enumerate() {
                if petgraph::algo::has_path_connecting(&reachability_graph, node_indices[from_idx], node_indices[to_idx], None) {
                    // already known to be transitively reachable, don't need to check for direct reachability
                    continue
                }
                // check whether the target state is directly reachable from the source state
                let is_reachable = match (from, to) {
                    (GlobalState { age: Age::Child, time_of_day: TimeOfDay::Noon }, GlobalState { age: Age::Child, time_of_day: TimeOfDay::Noon }) |
                    (GlobalState { age: Age::Child, time_of_day: TimeOfDay::Dampe }, GlobalState { age: Age::Child, time_of_day: TimeOfDay::Dampe }) |
                    (GlobalState { age: Age::Child, time_of_day: TimeOfDay::Midnight }, GlobalState { age: Age::Child, time_of_day: TimeOfDay::Midnight }) |
                    (GlobalState { age: Age::Adult, time_of_day: TimeOfDay::Noon }, GlobalState { age: Age::Adult, time_of_day: TimeOfDay::Noon }) |
                    (GlobalState { age: Age::Adult, time_of_day: TimeOfDay::Dampe }, GlobalState { age: Age::Adult, time_of_day: TimeOfDay::Dampe }) |
                    (GlobalState { age: Age::Adult, time_of_day: TimeOfDay::Midnight }, GlobalState { age: Age::Adult, time_of_day: TimeOfDay::Midnight }) => {
                        // The two states are identical, which is covered by the `has_path_connecting` check.
                        unreachable!()
                    }
                    (GlobalState { age: Age::Child, time_of_day: TimeOfDay::Noon }, GlobalState { age: Age::Adult, time_of_day: TimeOfDay::Noon }) |
                    (GlobalState { age: Age::Child, time_of_day: TimeOfDay::Dampe }, GlobalState { age: Age::Adult, time_of_day: TimeOfDay::Dampe }) |
                    (GlobalState { age: Age::Child, time_of_day: TimeOfDay::Midnight }, GlobalState { age: Age::Adult, time_of_day: TimeOfDay::Midnight }) => {
                        // No items or entrances shuffled yet, so Door of Time can be opened as child.
                        true
                    }
                    (GlobalState { age: Age::Adult, time_of_day: TimeOfDay::Noon }, GlobalState { age: Age::Child, time_of_day: TimeOfDay::Noon }) |
                    (GlobalState { age: Age::Adult, time_of_day: TimeOfDay::Dampe }, GlobalState { age: Age::Child, time_of_day: TimeOfDay::Dampe }) |
                    (GlobalState { age: Age::Adult, time_of_day: TimeOfDay::Midnight }, GlobalState { age: Age::Child, time_of_day: TimeOfDay::Midnight }) => {
                        // Starting age is always child, so we assume that if the player was able to bypass the Door of Time as child, they can do so again as adult.
                        // This will technically not be safe once we start shuffling items since adult requires items to skip the DoT, but it's required to make pretty much any logic work, so the “know what you're doing if you use glitches” rule applies.
                        true
                    }
                    (GlobalState { age: Age::Child, time_of_day: _ }, GlobalState { age: Age::Child, time_of_day: _ }) |
                    (GlobalState { age: Age::Adult, time_of_day: _ }, GlobalState { age: Age::Adult, time_of_day: _ }) => {
                        // No items or entrances shuffled yet, so both ages have access to regions where time passes, e.g. Hyrule Field.
                        true
                    }
                    (_, _) => false,
                };

                if is_reachable {
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
    loop {
        let mut progress_made = false;
        for world_region_access in &mut region_access {
            for (region, states) in world_region_access.clone() {
                for (vanilla_target, access) in region.info().exits {
                    for state in &states {
                        if !world_region_access.get(&vanilla_target).is_some_and(|already_reachable_states| already_reachable_states.contains(state)) && access(state) {
                            world_region_access.entry(vanilla_target).or_default().insert(*state);
                            progress_made = true;
                        }
                    }
                }
            }
        }
        if !progress_made { break }
    }
    // Search completed, check if we can beat the game.
    region_access.into_iter().all(|world_region_access| {
        // needs to be child to collect Zelda's Lullaby, which is required to beat the Shadow temple
        world_region_access.get(&Region::Root).is_some_and(|states| states.iter().any(|state| state.age == Age::Child))
        // needs to be able to reach Ganon
        && world_region_access.get(&Region::GanondorfBossRoom).is_some()
    }) //TODO different win conditions, e.g. ALR, no logic, Triforce Hunt, Bingo
}
