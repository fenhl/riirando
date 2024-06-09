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

"Queen Gohma Boss Room" {
    time_of_day: None,
    exits: {
        "Deku Tree": true,
        "Kokiri Forest": true, //TODO items required to defeat Gohma, separate region for Kokiri Forest near Deku Tree
    },
}

"Phantom Ganon Boss Room" {
    time_of_day: None,
    exits: {
        "Forest Temple": false,
        "Sacred Forest Meadow": true, //TODO item requirements, patch exit in ER
    },
}

"King Dodongo Boss Room" {
    time_of_day: None,
    exits: {
        "Dodongos Cavern": true,
        "Death Mountain Trail": true, //TODO item requirements
    },
}

"Volvagia Boss Room" {
    time_of_day: None,
    exits: {
        "Fire Temple": false,
        "Death Mountain Crater": true, //TODO item requirements, DMC point-to-point logic with health logic
    },
}

"Barinade Boss Room" {
    time_of_day: None,
    exits: {
        "Jabu Jabus Belly": false,
        "Zoras Fountain": is_child, //TODO item requirements
    },
}

"Morpha Boss Room" {
    time_of_day: None,
    exits: {
        "Water Temple": false,
        "Lake Hylia": true, //TODO item/trick requirements
    },
}

"Bongo Bongo Boss Room" {
    time_of_day: None,
    exits: {
        "Shadow Temple": false,
        "Graveyard": true, //TODO item requirements, separate region for Nocturne warp pad
    },
}

"Twinrova Boss Room" {
    time_of_day: None,
    exits: {
        "Spirit Temple": false,
        "Desert Colossus": is_adult, //TODO item requirements
    },
}
