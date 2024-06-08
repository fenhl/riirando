"Root" {
    time_of_day: None,
    exits: {
        "Kokiri Forest": is_child, //TODO check current savewarp (global state)
        "Temple of Time": is_adult, //TODO check current savewarp (global state)
        //TODO warp songs, dungeon savewarps
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
        "Lost Woods Bridge": true,
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
        "Market": is_child || at_night,
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

"Inside Ganons Castle" {
    time_of_day: None,
    exits: {
        "Castle Grounds": true, //TODO require rainbow bridge, add separate region for castle grounds from Ganon's Castle
        "Ganons Tower": true, //TODO require trials
    },
}

"Ganons Tower" {
    time_of_day: None,
    exits: {
        "Inside Ganons Castle": true,
        "Ganondorf Boss Room": true, //TODO require Ganon boss key
    },
}

"Ganondorf Boss Room" {
    time_of_day: None,
}

"Kokiri Forest" {
    time_of_day: Static,
    exits: {
        "Deku Tree": is_child, //TODO require Kokiri Sword and Deku Shield
        "Lost Woods": true,
        "Lost Woods Bridge": true, //TODO require Deku Tree Clear or adult
    },
}

"DekuTree" {
    time_of_day: None,
    exits: {
        "Kokiri Forest": true, //TODO separate region for Kokiri Forest near Deku Tree
        "Queen Gohma Boss Room": true, //TODO required items
    },
}

"Queen Gohma Boss Room" {
    time_of_day: None,
    exits: {
        "Deku Tree": true,
        "Kokiri Forest": true, //TODO items required to defeat Gohma, separate region for Kokiri Forest near Deku Tree
    },
}

"Lost Woods" {
    time_of_day: Static,
    exits: {
        "Kokiri Forest": true,
        "Lost Woods Bridge": is_adult, //TODO item requirements
        "Sacred Forest Meadow": true, //TODO item requirements for adult
    },
}

"Lost Woods Bridge" {
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

"Forest Temple" {
    time_of_day: None,
    exits: {
        "Sacred Forest Meadow": true,
        "Phantom Ganon Boss Room": true, //TODO
    },
}

"Phantom Ganon Boss Room" {
    time_of_day: None,
    exits: {
        "Forest Temple": false,
        "Sacred Forest Meadow": true, //TODO item requirements, patch exit in ER
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

"Dodongos Cavern" {
    time_of_day: None,
    exits: {
        "Death Mountain Trail": true,
        "King Dodongo Boss Room": true, //TODO item requirements
    },
}

"King Dodongo Boss Room" {
    time_of_day: None,
    exits: {
        "Dodongos Cavern": true,
        "Death Mountain Trail": true, //TODO item requirements
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

"Fire Temple" {
    time_of_day: None,
    exits: {
        "Death Mountain Crater": true, //TODO DMC point-to-point logic with health logic
        "Volvagia Boss Room": true, //TODO item requirements
    },
}

"Volvagia Boss Room" {
    time_of_day: None,
    exits: {
        "Fire Temple": false,
        "Death Mountain Crater": true, //TODO item requirements, DMC point-to-point logic with health logic
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

"Jabu Jabus Belly" {
    time_of_day: None,
    exits: {
        "Zoras Fountain": true,
        "Barinade Boss Room": true, //TODO item requirements
    },
}

"Barinade Boss Room" {
    time_of_day: None,
    exits: {
        "Jabu Jabus Belly": false,
        "Zoras Fountain": is_child, //TODO item requirements
    },
}

"Ice Cavern" {
    time_of_day: None,
    exits: {
        "Zoras Fountain": true,
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

"Water Temple" {
    time_of_day: None,
    exits: {
        "Lake Hylia": true,
        "Morpha Boss Room": is_adult, //TODO item requirements
    },
}

"Morpha Boss Room" {
    time_of_day: None,
    exits: {
        "Water Temple": false,
        "Lake Hylia": true, //TODO item/trick requirements
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

"Bottom of the Well" {
    time_of_day: None,
    exits: {
        "Kakariko Village": true,
    },
}

"Graveyard" {
    time_of_day: Static,
    exits: {
        "Kakariko Village": true, //TODO separate exit for Dampé race,
        "Shadow Temple": true, //TODO separate region for Nocturne warp pad
    },
}

"Shadow Temple" {
    time_of_day: None,
    exits: {
        "Graveyard": true,
        "Bongo Bongo Boss Room": is_adult, //TODO item requirements
    },
}

"Bongo Bongo Boss Room" {
    time_of_day: None,
    exits: {
        "Shadow Temple": false,
        "Graveyard": true, //TODO item requirements, separate region for Nocturne warp pad
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
    time_of_day: Static,
    exits: {
        "Gerudo Fortress": true, //TODO separate regions for the hideout sections
    },
}

"Gerudo Training Ground" {
    time_of_day: None,
    exits: {
        "Gerudo Fortress": true,
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

"Spirit Temple" {
    time_of_day: None,
    exits: {
        "Desert Colossus": true, //TODO separate exits for Requiem check exit and hands
        "Twinrova Boss Room": is_adult, //TODO item requirements
    },
}

"Twinrova Boss Room" {
    time_of_day: None,
    exits: {
        "Spirit Temple": false,
        "Desert Colossus": is_adult, //TODO item requirements
    },
}
