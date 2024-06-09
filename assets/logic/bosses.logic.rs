"Queen Gohma Boss Room" {
    savewarp: "Deku Tree",
    time_of_day: None,
    exits: {
        "Deku Tree": true,
        "Kokiri Forest": true, //TODO items required to defeat Gohma, separate region for Kokiri Forest near Deku Tree
    },
}

"King Dodongo Boss Room" {
    savewarp: "Dodongos Cavern",
    time_of_day: None,
    exits: {
        "Dodongos Cavern": true,
        "Death Mountain Trail": true, //TODO item requirements
    },
}

"Barinade Boss Room" {
    savewarp: "Jabu Jabus Belly",
    time_of_day: None,
    exits: {
        "Jabu Jabus Belly": false,
        "Zoras Fountain": is_child, //TODO item requirements
    },
}

"Phantom Ganon Boss Room" {
    savewarp: "Forest Temple",
    time_of_day: None,
    exits: {
        "Forest Temple": false,
        "Sacred Forest Meadow": true, //TODO item requirements, patch exit in ER
    },
}

"Volvagia Boss Room" {
    savewarp: "Fire Temple",
    time_of_day: None,
    exits: {
        "Fire Temple": false,
        "Death Mountain Crater": true, //TODO item requirements, DMC point-to-point logic with health logic
    },
}

"Morpha Boss Room" {
    savewarp: "Water Temple",
    time_of_day: None,
    exits: {
        "Water Temple": false,
        "Lake Hylia": true, //TODO item/trick requirements
    },
}

"Bongo Bongo Boss Room" {
    savewarp: "Shadow Temple",
    time_of_day: None,
    exits: {
        "Shadow Temple": false,
        "Graveyard": true, //TODO item requirements, separate region for Nocturne warp pad
    },
}

"Twinrova Boss Room" {
    savewarp: "Spirit Temple",
    time_of_day: None,
    exits: {
        "Spirit Temple": false,
        "Desert Colossus": is_adult, //TODO item requirements
    },
}

"Ganons Tower" {
    savewarp: "Ganons Tower",
    time_of_day: None,
    exits: {
        "Inside Ganons Castle": true,
        "Ganondorf Boss Room": true, //TODO require Ganon boss key
    },
}

"Ganondorf Boss Room" {
    savewarp: "Ganons Tower",
    time_of_day: None,
}
