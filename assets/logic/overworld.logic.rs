"Root" {
    time_of_day: None,
    exits: {
        // savewarp exits are hardcoded
        "Temple of Time": PreludeOfLight,
        "Sacred Forest Meadow": MinuetOfForest,
        "Death Mountain Crater": BoleroOfFire, //TODO DMC point-to-point logic with health logic
        "Lake Hylia": SerenadeOfWater,
        "Graveyard Warp Pad Region": NocturneOfShadow,
        "Desert Colossus": RequiemOfSpirit,
    },
}

"Overworld" {
    time_of_day: None,
    exits: {
        "KF Links House": is_child,
        "Temple of Time": is_adult,
    },
}

"Hyrule Field" {
    time_of_day: Passes,
    exits: {
        "Lon Lon Ranch": true,
        // Can enter market entrance as child at night by waiting on the drawbridge.
        // This is in base logic since the wonderitems on top of the drawbridge show that waiting on the drawbridge until night was intended,
        // even if not necessarily to enter the market entrance.
        "Market Entrance": true,
        "Kakariko Village": true,
        "Zora River": true,
        "LW Bridge": true,
        "Lake Hylia": true,
        "Gerudo Valley": true,
    },
}

"Lon Lon Ranch" {
    time_of_day: Static,
    exits: {
        "Hyrule Field": true,
    },
}

"Market Entrance" {
    time_of_day: Static,
    exits: {
        "Hyrule Field": is_adult || at_day,
        "Market": true,
    },
}

"Market" {
    time_of_day: Static,
    exits: {
        "Market Entrance": true,
        "Market Back Alley": is_child,
        "Temple of Time Entrance": true,
        "Castle Grounds": true,
    },
}

"Market Back Alley" {
    time_of_day: Static,
    exits: {
        "Market": true,
    },
}

"Temple of Time Entrance" {
    time_of_day: Static,
    exits: {
        "Market": true,
        "Temple of Time": true,
    },
}

"Temple of Time" {
    time_of_day: Static,
    exits: {
        "Temple of Time Entrance": true,
        // We assume that if the player was able to bypass the Door of Time as the starting age, they can do so again as the non-starting age.
        // This is technically not safe since items are required to skip the DoT, but it's required to make pretty much any logic work, so the “know what you're doing if you use glitches” rule applies.
        "Beyond Door of Time": true, //TODO check for non-starting age or can_play(SongOfTime)
    },
}

"Beyond Door of Time" {
    time_of_day: None,
    exits: {
        // We assume that if the player was able to bypass the Door of Time, they can do so again in reverse as both ages.
        "Temple of Time": true,
    },
}

"Castle Grounds" {
    time_of_day: None,
    exits: {
        "Market": is_child || at_dampe_time,
        "Hyrule Castle": is_child,
        "Outside Ganons Castle": is_adult,
    },
}

"Hyrule Castle" {
    time_of_day: Passes,
    exits: {
        "Castle Grounds": true,
    },
}

"Outside Ganons Castle" {
    time_of_day: OutsideGanonsCastle,
    exits: {
        "Castle Grounds": true,
        "Inside Ganons Castle": at_dampe_time, //TODO require rainbow bridge
    },
}

"Kokiri Forest" {
    time_of_day: Static,
    items: {
        KokiriSword: is_child,
        GoldSkulltulaToken: is_child && can_child_attack && at_night, // KF GS Know It All House //TODO access via starting ToD in Require Gohma?
        GoldSkulltulaToken: Bugs && can_child_attack, // KF GS Bean Patch
        GoldSkulltulaToken: is_adult && at_night && Hookshot, // KF GS House of Twins
    },
    exits: {
        "KF Links House": true,
        "KF Midos House": true,
        "KF Sarias House": true,
        "KF House of Twins": true,
        "KF Know It All House": true,
        "KF Shop": true,
        "KF Outside Deku Tree": is_adult || (KokiriSword && DekuShield), //TODO access with open Deku, turn sword/shield access into event for dungeon ER
        "Lost Woods": true,
        "LW Bridge From Forest": is_adult || "Deku Tree Clear",
        "KF Storms Grotto": can_open_storm_grotto,
    },
}

"KF Outside Deku Tree" {
    time_of_day: Static,
    items: {
        // The Babas despawn for Adult on forest temple completion. The original implementation keeps them in logic when entrance rando is off, but this doesn't seem to be safe with the shortcut enabled.
        //TODO implement this fix? https://discord.com/channels/274180765816848384/759842532963385354/1249325077351104615
        DekuSticks: KokiriSword || Boomerang,
    },
    exits: {
        "Deku Tree Lobby": is_child, //TODO open as adult with dungeon ER
        "Kokiri Forest": is_adult || (KokiriSword && DekuShield), //TODO access with open Deku, turn sword/shield access into event
    },
}

"KF Links House" {
    savewarp: "KF Links House",
    time_of_day: Static,
    exits: {
        "Kokiri Forest": true,
    },
}

"KF Midos House" {
    time_of_day: Static,
    exits: {
        "Kokiri Forest": true,
    },
}

"KF Sarias House" {
    time_of_day: Static,
    exits: {
        "Kokiri Forest": true,
    },
}

"KF House of Twins" {
    time_of_day: Static,
    exits: {
        "Kokiri Forest": true,
    },
}

"KF Know It All House" {
    time_of_day: Static,
    exits: {
        "Kokiri Forest": true,
    },
}

"KF Shop" {
    time_of_day: Static,
    items: {
        DekuShield: can_pay(40),
        DekuNuts: can_pay(15),
        DekuSticks: can_pay(10),
        DekuSeeds: can_pay(30),
        Arrows: can_pay(20),
        RecoveryHearts: can_pay(10),
    },
    exits: {
        "Kokiri Forest": true,
    },
}

"Lost Woods" {
    time_of_day: Static,
    items: {
        PieceOfHeart: is_child && SariasSong, // LW Skull Kid
        OddMushroom: Cojiro, //TODO disable trade timers/reverts with relevant ER settings
        PoachersSaw: OddPotion, //TODO disable trade timers/reverts with relevant ER settings
        PieceOfHeart: is_child && Ocarina && OcarinaAButton && OcarinaCDownButton && OcarinaCRightButton && OcarinaCLeftButton && OcarinaCUpButton, // LW Ocarina Memory Game
        Slingshot: Slingshot,
        GoldSkulltulaToken: Bugs && can_child_attack, // LW GS Bean Patch Near Bridge
        Bugs: is_child && can_cut_shrubs && Bottle,
    },
    exits: {
        "Kokiri Forest": true,
        "GC Woods Warp": true,
        "LW Bridge": is_adult && (HoverBoots || Longshot || here(can_plant_bean)),
        "LW Underwater Entrance": is_child && (can_dive || Boomerang),
        "Sacred Forest Meadow": true, //TODO item requirements for adult
    },
}

"LW Bridge" {
    time_of_day: Static,
    exits: {
        "Hyrule Field": true,
        "Kokiri Forest": true, //TODO hack to move entrance coords past Pokey in Closed Forest
        "Lost Woods": is_adult, //TODO require longshot
    },
}

"Sacred Forest Meadow" {
    time_of_day: Static,
    exits: {
        "Lost Woods": true,
        "Forest Temple": is_adult, //TODO require hookshot, separate region
    },
}

"Death Mountain Trail" {
    time_of_day: Passes,
    exits: {
        "Kakariko Village": true, //TODO gate behavior/trick, separate exit for the owl
        "Dodongos Cavern": true, //TODO item requirements for child access
        "Goron City": true,
        "Death Mountain Crater": true, //TODO DMC point-to-point logic with health logic
    },
}

"Goron City" {
    time_of_day: None,
    exits: {
        "Death Mountain Trail": true,
        "Death Mountain Crater": is_adult, //TODO separate region for Darunia's chamber, DMC point-to-point logic with health logic
        "Lost Woods": true, //TODO item requirements
    },
}

"Death Mountain Crater" {
    time_of_day: Static,
    exits: { //TODO DMC point-to-point logic with health logic
        "Goron City": true,
        "Death Mountain Trail": true,
        "Fire Temple": is_adult,
    },
}

"Zora River" {
    time_of_day: Passes,
    exits: {
        "Hyrule Field": true,
        "Lost Woods": true, //TODO item requirements
        "Zoras Domain": true, //TODO item requirements
    },
}

"Zoras Domain" {
    time_of_day: Static,
    exits: {
        "Zora River": true,
        "Lake Hylia": is_child, //TODO separate region for returning as adult with iron boots (and trick?)
        "Zoras Fountain": true, //TODO event/setting/item/trick requirements
    },
}

"Zoras Fountain" {
    time_of_day: Static,
    exits: {
        "Zoras Domain": true,
        "Jabu Jabus Belly": is_child, //TODO item requirements
        "Ice Cavern": is_adult,
    },
}

"Lake Hylia" {
    time_of_day: Passes,
    exits: {
        "Hyrule Field": true, //TODO separate exit for the owl
        "Zoras Domain": is_child, //TODO item requirements
        "Water Temple": is_adult, //TODO item/setting requirements, add water level toggle
    },
}

"Kakariko Village" {
    time_of_day: Static,
    exits: {
        "Hyrule Field": true,
        "Death Mountain Trail": true, //TODO event/age requirements
        "Graveyard": true,
        "Bottom of the Well": is_child, //TODO event requirement, patch to allow access as adult in dungeon ER
    },
}

"Graveyard" {
    time_of_day: Static,
    exits: {
        "Kakariko Village": true, //TODO separate exit for Dampé race
    },
}

"Graveyard Warp Pad Region" {
    time_of_day: Static,
    exits: {
        "Graveyard": true,
        "Shadow Temple": true, //TODO item/trick requirements
    },
}

"Gerudo Valley" {
    time_of_day: Passes,
    exits: {
        "Hyrule Field": true,
        "Lake Hylia": true,
        "Gerudo Fortress": is_adult, //TODO item/event/setting requirements
    },
}

"Gerudo Fortress" {
    time_of_day: Static,
    exits: {
        "Gerudo Valley": true,
        "Thieves Hideout": true, //TODO separate regions for the hideout sections
        "Gerudo Training Ground": true, //TODO item requirement, fix possible ER softlock due to entry fee
        "Haunted Wasteland": true, //TODO item requirement
    },
}

"Thieves Hideout" {
    savewarp: "Thieves Hideout",
    time_of_day: Static,
    exits: {
        "Gerudo Fortress": true, //TODO separate regions for the hideout sections
    },
}

"Haunted Wasteland" {
    time_of_day: Static,
    exits: {
        "Gerudo Fortress": is_adult, //TODO separate wasteland regions
        "Desert Colossus": is_adult, //TODO separate wasteland regions, item/trick requirements
    },
}

"Desert Colossus" {
    time_of_day: Passes,
    exits: {
        "Haunted Wasteland": true,
        "Spirit Temple": true,
    },
}
