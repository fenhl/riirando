use {
    std::collections::HashMap,
    collect_mac::collect,
    riirando_common::*,
    crate::search::{
        Age,
        GlobalState,
        Inventory,
        TimeOfDay,
    },
};

type Access = fn(&GlobalState, &Inventory) -> bool;

pub(crate) struct RegionInfo {
    pub(crate) savewarp: Savewarp,
    pub(crate) time_of_day: TimeOfDayBehavior,
    pub(crate) items: HashMap<Item, Vec<Access>>,
    pub(crate) exits: HashMap<Region, Access>,
}

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct AnonymousEvent {}

riirando_macros::regions!();
