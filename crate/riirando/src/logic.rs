use {
    std::collections::HashMap,
    collect_mac::collect,
    crate::search::{
        Age,
        GlobalState,
    },
};

type Access = fn(&GlobalState) -> bool;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Region {
    Root,
    GanondorfBossRoom,
}

pub(crate) struct RegionInfo {
    pub(crate) exits: HashMap<Region, Access>,
}

impl Region {
    pub(crate) fn info(&self) -> RegionInfo {
        match self {
            Self::Root => RegionInfo {
                exits: collect![as HashMap<Region, Access>:
                    Self::GanondorfBossRoom => (|state| state.age == Age::Adult) as Access,
                ],
            },
            Self::GanondorfBossRoom => RegionInfo {
                exits: HashMap::default(),
            },
        }
    }
}
