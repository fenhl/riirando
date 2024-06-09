use {
    std::collections::HashMap,
    collect_mac::collect,
    enumset::EnumSet,
    riirando_common::*,
    crate::search::{
        Age,
        GlobalState,
        TimeOfDay,
    },
};

type Access = fn(&GlobalState, &EnumSet<Item>) -> bool;

pub(crate) struct RegionInfo {
    pub(crate) savewarp: Savewarp,
    pub(crate) time_of_day: TimeOfDayBehavior,
    pub(crate) items: HashMap<Item, Access>,
    pub(crate) exits: HashMap<Region, Access>,
}

riirando_macros::regions!();
