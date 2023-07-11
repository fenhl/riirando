use {
    enum_iterator::{
        Sequence,
        all,
    },
    enumset::{
        EnumSet,
        EnumSetType,
    },
    itertools::Itertools as _,
    petgraph::matrix_graph::DiMatrix,
};

#[derive(EnumSetType, Sequence)]
enum Age {
    Child,
    Adult,
}

#[derive(Clone, Copy, Sequence)]
struct GlobalState {
    age: Age,
    //TODO time of day, spawn position
}

#[derive(Default)]
struct GlobalStateSet {
    ages: EnumSet<Age>,
    //TODO times of day, spawn positions
}

impl FromIterator<GlobalState> for GlobalStateSet {
    fn from_iter<T: IntoIterator<Item = GlobalState>>(iter: T) -> Self {
        let mut set = Self::default();
        for GlobalState { age } in iter {
            set.ages.insert(age);
        }
        set
    }
}

pub(crate) fn can_win() -> bool {
    let mut reachability_graph = DiMatrix::<_, _>::with_capacity(GlobalState::CARDINALITY);
    let node_indices = all::<GlobalState>().map(|state| reachability_graph.add_node(state)).collect_vec();
    for (from_idx, from) in all().enumerate() {
        for (to_idx, to) in all().enumerate() {
            if from_idx == to_idx { continue } // don't need to check whether a state is reachable from itself
            // check whether the target state is directly reachable from the source state
            let is_reachable = match (from, to) {
                (GlobalState { age: Age::Child }, GlobalState { age: Age::Child }) => unreachable!(),
                (GlobalState { age: Age::Child }, GlobalState { age: Age::Adult }) => true, // Door of Time can be opened as child
                (GlobalState { age: Age::Adult }, GlobalState { age: Age::Child }) => true, // we assume that if the player was able to bypass the Door of Time as child, they can do so again as adult
                (GlobalState { age: Age::Adult }, GlobalState { age: Age::Adult }) => unreachable!(),
            };
            if is_reachable {
                reachability_graph.add_edge(node_indices[from_idx], node_indices[to_idx], ());
            }
        }
    }
    let mut dfs_space = petgraph::algo::DfsSpace::new(&reachability_graph);
    let reachable_states = all()
        .enumerate()
        .filter(|&(to_idx, _)| node_indices.iter().all(|&from_idx| petgraph::algo::has_path_connecting(&reachability_graph, from_idx, node_indices[to_idx], Some(&mut dfs_space))))
        .map(|(_, to)| to)
        .collect::<GlobalStateSet>();
    reachable_states.ages.contains(Age::Adult) // needs to be adult to reach Ganon //TODO different win conditions, e.g. ALR, no logic, Triforce Hunt, Bingo
}
