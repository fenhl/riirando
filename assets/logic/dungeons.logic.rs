"Inside Ganons Castle" {
    time_of_day: None,
    exits: {
        "Castle Grounds": true, //TODO require rainbow bridge, add separate region for castle grounds from Ganon's Castle
        "Ganons Tower": true, //TODO require trials
    },
}

"Deku Tree" {
    time_of_day: None,
    exits: {
        "Kokiri Forest": true, //TODO separate region for Kokiri Forest near Deku Tree
        "Queen Gohma Boss Room": true, //TODO required items
    },
}

"Forest Temple" {
    time_of_day: None,
    exits: {
        "Sacred Forest Meadow": true,
        "Phantom Ganon Boss Room": true, //TODO
    },
}

"Dodongos Cavern" {
    time_of_day: None,
    exits: {
        "Death Mountain Trail": true,
        "King Dodongo Boss Room": true, //TODO item requirements
    },
}

"Fire Temple" {
    time_of_day: None,
    exits: {
        "Death Mountain Crater": true, //TODO DMC point-to-point logic with health logic
        "Volvagia Boss Room": true, //TODO item requirements
    },
}

"Jabu Jabus Belly" {
    time_of_day: None,
    exits: {
        "Zoras Fountain": true,
        "Barinade Boss Room": true, //TODO item requirements
    },
}

"Ice Cavern" {
    time_of_day: None,
    exits: {
        "Zoras Fountain": true,
    },
}

"Water Temple" {
    time_of_day: None,
    exits: {
        "Lake Hylia": true,
        "Morpha Boss Room": is_adult, //TODO item requirements
    },
}

"Bottom of the Well" {
    time_of_day: None,
    exits: {
        "Kakariko Village": true,
    },
}

"Shadow Temple" {
    time_of_day: None,
    exits: {
        "Graveyard": true,
        "Bongo Bongo Boss Room": is_adult, //TODO item requirements
    },
}

"Gerudo Training Ground" {
    time_of_day: None,
    exits: {
        "Gerudo Fortress": true,
    },
}

"Spirit Temple" {
    time_of_day: None,
    exits: {
        "Desert Colossus": true, //TODO separate exits for Requiem check exit and hands
        "Twinrova Boss Room": is_adult, //TODO item requirements
    },
}
