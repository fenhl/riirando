use {
    std::collections::HashMap,
    collect_mac::collect,
    riirando_common::*,
    crate::search::{
        Age,
        GlobalState,
        TimeOfDay,
    },
};

type Access = fn(&GlobalState) -> bool;

pub(crate) struct RegionInfo {
    pub(crate) savewarp: Savewarp,
    pub(crate) time_of_day: TimeOfDayBehavior,
    pub(crate) exits: HashMap<Region, Access>,
}

riirando_macros::regions!();
