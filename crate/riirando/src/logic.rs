use {
    std::collections::HashMap,
    collect_mac::collect,
    crate::search::{
        Age,
        GlobalState,
        TimeOfDay,
    },
};

type Access = fn(&GlobalState) -> bool;

const ALWAYS: Access = |_| true;
const NEVER: Access = |_| false;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Region {
    Root,
    HyruleField,
    LonLonRanch,
    MarketEntrance,
    Market,
    MarketBackAlley,
    TempleOfTimeEntrance,
    TempleOfTime,
    BeyondDoorOfTime,
    CastleGrounds,
    HyruleCastle,
    OutsideGanonsCastle,
    InsideGanonsCastle,
    GanonsTower,
    GanondorfBossRoom,
    KokiriForest,
    DekuTree,
    QueenGohmaBossRoom,
    LostWoods,
    LostWoodsBridge,
    SacredForestMeadow,
    ForestTemple,
    PhantomGanonBossRoom,
    DeathMountainTrail,
    DodongosCavern,
    KingDodongoBossRoom,
    GoronCity,
    DeathMountainCrater,
    FireTemple,
    VolvagiaBossRoom,
    ZoraRiver,
    ZorasDomain,
    ZorasFountain,
    JabuJabusBelly,
    BarinadeBossRoom,
    IceCavern,
    LakeHylia,
    WaterTemple,
    MorphaBossRoom,
    KakarikoVillage,
    BottomOfTheWell,
    Graveyard,
    ShadowTemple,
    BongoBongoBossRoom,
    GerudoValley,
    GerudoFortress,
    ThievesHideout,
    GerudoTrainingGround,
    HauntedWasteland,
    DesertColossus,
    SpiritTemple,
    TwinrovaBossRoom,
}

pub(crate) enum TimeOfDayBehavior {
    /// Cannot alter time of day in this region. Used for dungeons as well as helper regions like Root.
    None,
    /// Time does not pass but can be set to noon or midnight using the Sun's Song. This reloads the scene.
    Static,
    /// Time passes normally and can be set to any value simply by waiting.
    Passes,
    /// Special behavior for Ganon's castle grounds which force time of day to Dampé time.
    OutsideGanonsCastle,
}

pub(crate) struct RegionInfo {
    pub(crate) time_of_day: TimeOfDayBehavior,
    pub(crate) exits: HashMap<Region, Access>,
}

impl Region {
    pub(crate) fn info(&self) -> RegionInfo {
        match self {
            Self::Root => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::KokiriForest => (|state| state.age == Age::Child) as Access, //TODO check current savewarp (global state)
                    Self::TempleOfTime => (|state| state.age == Age::Adult) as Access, //TODO check current savewarp (global state)
                    //TODO warp songs, dungeon savewarps
                ],
            },
            Self::HyruleField => RegionInfo {
                time_of_day: TimeOfDayBehavior::Passes,
                exits: collect![
                    Self::LonLonRanch => ALWAYS,
                    // Can enter market entrance as child at night by waiting on the drawbridge.
                    // This is in base logic since the wonderitems on top of the drawbridge show that waiting on the drawbridge until night was intended,
                    // even if not necessarily to enter the market entrance.
                    Self::MarketEntrance => ALWAYS,
                    Self::KakarikoVillage => ALWAYS,
                    Self::ZoraRiver => ALWAYS,
                    Self::LostWoodsBridge => ALWAYS,
                    Self::LakeHylia => ALWAYS,
                    Self::GerudoValley => ALWAYS,
                ],
            },
            Self::LonLonRanch => RegionInfo {
                time_of_day: TimeOfDayBehavior::Static,
                exits: collect![
                    Self::HyruleField => ALWAYS,
                ],
            },
            Self::MarketEntrance => RegionInfo {
                time_of_day: TimeOfDayBehavior::Static,
                exits: collect![
                    Self::HyruleField => (|state| state.time_of_day.is_day() || state.age == Age::Adult) as Access,
                    Self::Market => ALWAYS,
                ],
            },
            Self::Market => RegionInfo {
                time_of_day: TimeOfDayBehavior::Static,
                exits: collect![
                    Self::MarketEntrance => ALWAYS,
                    Self::MarketBackAlley => (|state| state.age == Age::Child) as Access,
                    Self::TempleOfTimeEntrance => ALWAYS,
                    Self::CastleGrounds => ALWAYS,
                ],
            },
            Self::MarketBackAlley => RegionInfo {
                time_of_day: TimeOfDayBehavior::Static,
                exits: collect![
                    Self::Market => ALWAYS,
                ],
            },
            Self::TempleOfTimeEntrance => RegionInfo {
                time_of_day: TimeOfDayBehavior::Static,
                exits: collect![
                    Self::Market => ALWAYS,
                    Self::TempleOfTime => ALWAYS,
                ],
            },
            Self::TempleOfTime => RegionInfo {
                time_of_day: TimeOfDayBehavior::Static,
                exits: collect![
                    Self::TempleOfTimeEntrance => ALWAYS,
                    // We assume that if the player was able to bypass the Door of Time as the starting age, they can do so again as the non-starting age.
                    // This is technically not safe since items are required to skip the DoT, but it's required to make pretty much any logic work, so the “know what you're doing if you use glitches” rule applies.
                    Self::BeyondDoorOfTime => ALWAYS, //TODO check for non-starting age or can_play(SongOfTime)
                ],
            },
            Self::BeyondDoorOfTime => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    // We assume that if the player was able to bypass the Door of Time, they can do so again in reverse as both ages.
                    Self::TempleOfTime => ALWAYS,
                ],
            },
            Self::CastleGrounds => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::Market => (|state| state.age == Age::Child || state.time_of_day == TimeOfDay::Dampe) as Access,
                    Self::HyruleCastle => (|state| state.age == Age::Child) as Access,
                    Self::OutsideGanonsCastle => (|state| state.age == Age::Adult) as Access,
                ],
            },
            Self::HyruleCastle => RegionInfo {
                time_of_day: TimeOfDayBehavior::Passes,
                exits: collect![
                    Self::CastleGrounds => ALWAYS,
                ],
            },
            Self::OutsideGanonsCastle => RegionInfo {
                time_of_day: TimeOfDayBehavior::OutsideGanonsCastle,
                exits: collect![
                    Self::CastleGrounds => ALWAYS,
                    Self::InsideGanonsCastle => (|state| state.time_of_day == TimeOfDay::Dampe) as Access, //TODO require rainbow bridge
                ],
            },
            Self::InsideGanonsCastle => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::CastleGrounds => ALWAYS, //TODO require rainbow bridge, add separate region for castle grounds from Ganon's Castle
                    Self::GanonsTower => ALWAYS, //TODO require trials
                ],
            },
            Self::GanonsTower => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::InsideGanonsCastle => ALWAYS,
                    Self::GanondorfBossRoom => ALWAYS, //TODO require Ganon boss key
                ],
            },
            Self::GanondorfBossRoom => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: HashMap::default(),
            },
            Self::KokiriForest => RegionInfo {
                time_of_day: TimeOfDayBehavior::Static,
                exits: collect![
                    Self::DekuTree => (|state| state.age == Age::Child) as Access, //TODO require Kokiri Sword and Deku Shield
                    Self::LostWoods => ALWAYS,
                    Self::LostWoodsBridge => ALWAYS, //TODO require Deku Tree Clear or adult
                ],
            },
            Self::DekuTree => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::KokiriForest => ALWAYS, //TODO separate region for Kokiri Forest near Deku Tree
                    Self::QueenGohmaBossRoom => ALWAYS, //TODO required items
                ],
            },
            Self::QueenGohmaBossRoom => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::DekuTree => ALWAYS,
                    Self::KokiriForest => ALWAYS, //TODO items required to defeat Gohma, separate region for Kokiri Forest near Deku Tree
                ],
            },
            Self::LostWoods => RegionInfo {
                time_of_day: TimeOfDayBehavior::Static,
                exits: collect![
                    Self::KokiriForest => ALWAYS,
                    Self::LostWoodsBridge => (|state| state.age == Age::Adult) as Access, //TODO item requirements
                    Self::SacredForestMeadow => ALWAYS, //TODO item requirements for adult
                ],
            },
            Self::LostWoodsBridge => RegionInfo {
                time_of_day: TimeOfDayBehavior::Static,
                exits: collect![
                    Self::HyruleField => ALWAYS,
                    Self::KokiriForest => ALWAYS, //TODO hack to move entrance coords past Pokey in Closed Forest
                    Self::LostWoods => (|state| state.age == Age::Adult) as Access, //TODO require longshot
                ],
            },
            Self::SacredForestMeadow => RegionInfo {
                time_of_day: TimeOfDayBehavior::Static,
                exits: collect![
                    Self::LostWoods => ALWAYS,
                    Self::ForestTemple => (|state| state.age == Age::Adult) as Access, //TODO require hookshot, separate region
                ],
            },
            Self::ForestTemple => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::SacredForestMeadow => ALWAYS,
                    Self::PhantomGanonBossRoom => ALWAYS, //TODO
                ],
            },
            Self::PhantomGanonBossRoom => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::ForestTemple => NEVER,
                    Self::SacredForestMeadow => ALWAYS, //TODO item requirements, patch exit in ER
                ],
            },
            Self::DeathMountainTrail => RegionInfo {
                time_of_day: TimeOfDayBehavior::Passes,
                exits: collect![
                    Self::KakarikoVillage => ALWAYS, //TODO gate behavior/trick, separate exit for the owl
                    Self::DodongosCavern => ALWAYS, //TODO item requirements for child access
                    Self::GoronCity => ALWAYS,
                    Self::DeathMountainCrater => ALWAYS, //TODO DMC point-to-point logic with health logic
                ],
            },
            Self::DodongosCavern => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::DeathMountainTrail => ALWAYS,
                    Self::KingDodongoBossRoom => ALWAYS, //TODO item requirements
                ],
            },
            Self::KingDodongoBossRoom => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::DodongosCavern => ALWAYS,
                    Self::DeathMountainTrail => ALWAYS, //TODO item requirements
                ],
            },
            Self::GoronCity => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::DeathMountainTrail => ALWAYS,
                    Self::DeathMountainCrater => (|state| state.age == Age::Adult) as Access, //TODO separate region for Darunia's chamber, DMC point-to-point logic with health logic
                    Self::LostWoods => ALWAYS, //TODO item requirements
                ],
            },
            Self::DeathMountainCrater => RegionInfo {
                time_of_day: TimeOfDayBehavior::Static,
                exits: collect![ //TODO DMC point-to-point logic with health logic
                    Self::GoronCity => ALWAYS,
                    Self::DeathMountainTrail => ALWAYS,
                    Self::FireTemple => (|state| state.age == Age::Adult) as Access,
                ],
            },
            Self::FireTemple => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::DeathMountainCrater => ALWAYS, //TODO DMC point-to-point logic with health logic
                    Self::VolvagiaBossRoom => ALWAYS, //TODO item requirements
                ],
            },
            Self::VolvagiaBossRoom => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::FireTemple => NEVER,
                    Self::DeathMountainCrater => ALWAYS, //TODO item requirements, DMC point-to-point logic with health logic
                ],
            },
            Self::ZoraRiver => RegionInfo {
                time_of_day: TimeOfDayBehavior::Passes,
                exits: collect![
                    Self::HyruleField => ALWAYS,
                    Self::LostWoods => ALWAYS, //TODO item requirements
                    Self::ZorasDomain => ALWAYS, //TODO item requirements
                ],
            },
            Self::ZorasDomain => RegionInfo {
                time_of_day: TimeOfDayBehavior::Static,
                exits: collect![
                    Self::ZoraRiver => ALWAYS,
                    Self::LakeHylia => (|state| state.age == Age::Child) as Access, //TODO separate region for returning as adult with iron boots (and trick?)
                    Self::ZorasFountain => ALWAYS, //TODO event/setting/item/trick requirements
                ],
            },
            Self::ZorasFountain => RegionInfo {
                time_of_day: TimeOfDayBehavior::Static,
                exits: collect![
                    Self::ZorasDomain => ALWAYS,
                    Self::JabuJabusBelly => (|state| state.age == Age::Child) as Access, //TODO item requirements
                    Self::IceCavern => (|state| state.age == Age::Adult) as Access,
                ],
            },
            Self::JabuJabusBelly => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::ZorasFountain => ALWAYS,
                    Self::BarinadeBossRoom => ALWAYS, //TODO item requirements
                ],
            },
            Self::BarinadeBossRoom => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::JabuJabusBelly => NEVER,
                    Self::ZorasFountain => (|state| state.age == Age::Child) as Access, //TODO item requirements
                ],
            },
            Self::IceCavern => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::ZorasFountain => ALWAYS,
                ],
            },
            Self::LakeHylia => RegionInfo {
                time_of_day: TimeOfDayBehavior::Passes,
                exits: collect![
                    Self::HyruleField => ALWAYS, //TODO separate exit for the owl
                    Self::ZorasDomain => (|state| state.age == Age::Child) as Access, //TODO item requirements
                    Self::WaterTemple => (|state| state.age == Age::Adult) as Access, //TODO item/setting requirements, add water level toggle
                ],
            },
            Self::WaterTemple => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::LakeHylia => ALWAYS,
                    Self::MorphaBossRoom => (|state| state.age == Age::Adult) as Access, //TODO item requirements
                ],
            },
            Self::MorphaBossRoom => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::WaterTemple => NEVER,
                    Self::LakeHylia => ALWAYS, //TODO item/trick requirements
                ],
            },
            Self::KakarikoVillage => RegionInfo {
                time_of_day: TimeOfDayBehavior::Static,
                exits: collect![
                    Self::HyruleField => ALWAYS,
                    Self::DeathMountainTrail => ALWAYS, //TODO event/age requirements
                    Self::Graveyard => ALWAYS,
                    Self::BottomOfTheWell => (|state| state.age == Age::Child) as Access, //TODO event requirement, patch to allow access as adult in dungeon ER
                ],
            },
            Self::BottomOfTheWell => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::KakarikoVillage => ALWAYS,
                ],
            },
            Self::Graveyard => RegionInfo {
                time_of_day: TimeOfDayBehavior::Static,
                exits: collect![
                    Self::KakarikoVillage => ALWAYS, //TODO separate exit for Dampé race,
                    Self::ShadowTemple => ALWAYS, //TODO separate region for Nocturne warp pad
                ],
            },
            Self::ShadowTemple => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::Graveyard => ALWAYS,
                    Self::BongoBongoBossRoom => (|state| state.age == Age::Adult) as Access, //TODO item requirements
                ],
            },
            Self::BongoBongoBossRoom => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::ShadowTemple => NEVER,
                    Self::Graveyard => ALWAYS, //TODO item requirements, separate region for Nocturne warp pad
                ],
            },
            Self::GerudoValley => RegionInfo {
                time_of_day: TimeOfDayBehavior::Passes,
                exits: collect![
                    Self::HyruleField => ALWAYS,
                    Self::LakeHylia => ALWAYS,
                    Self::GerudoFortress => (|state| state.age == Age::Adult) as Access, //TODO item/event/setting requirements
                ],
            },
            Self::GerudoFortress => RegionInfo {
                time_of_day: TimeOfDayBehavior::Static,
                exits: collect![
                    Self::GerudoValley => ALWAYS,
                    Self::ThievesHideout => ALWAYS, //TODO separate regions for the hideout sections
                    Self::GerudoTrainingGround => ALWAYS, //TODO item requirement, fix possible ER softlock due to entry fee
                    Self::HauntedWasteland => ALWAYS, //TODO item requirement
                ],
            },
            Self::ThievesHideout => RegionInfo {
                time_of_day: TimeOfDayBehavior::Static,
                exits: collect![
                    Self::GerudoFortress => ALWAYS, //TODO separate regions for the hideout sections
                ],
            },
            Self::GerudoTrainingGround => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::GerudoFortress => ALWAYS,
                ],
            },
            Self::HauntedWasteland => RegionInfo {
                time_of_day: TimeOfDayBehavior::Static,
                exits: collect![
                    Self::GerudoFortress => (|state| state.age == Age::Adult) as Access, //TODO separate wasteland regions
                    Self::DesertColossus => (|state| state.age == Age::Adult) as Access, //TODO separate wasteland regions, item/trick requirements
                ],
            },
            Self::DesertColossus => RegionInfo {
                time_of_day: TimeOfDayBehavior::Passes,
                exits: collect![
                    Self::HauntedWasteland => ALWAYS,
                    Self::SpiritTemple => ALWAYS,
                ],
            },
            Self::SpiritTemple => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::DesertColossus => ALWAYS, //TODO separate exits for Requiem check exit and hands
                    Self::TwinrovaBossRoom => (|state| state.age == Age::Adult) as Access, //TODO item requirements
                ],
            },
            Self::TwinrovaBossRoom => RegionInfo {
                time_of_day: TimeOfDayBehavior::None,
                exits: collect![
                    Self::SpiritTemple => NEVER,
                    Self::DesertColossus => (|state| state.age == Age::Adult) as Access, //TODO item requirements
                ],
            },
        }
    }
}
