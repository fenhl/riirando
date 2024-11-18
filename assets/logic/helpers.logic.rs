fn Longshot() {
    (Hookshot, 2)
}

fn can_child_attack() {
    is_child && (Slingshot || Boomerang || DekuSticks || KokiriSword || has_explosives || DinsFire)
}

fn can_cut_shrubs() {
    is_adult || DekuSticks || KokiriSword || Boomerang || has_explosives
}

fn can_dive() {
    Scale
}

fn can_open_storm_grotto() {
    SongOfStorms && StoneOfAgony
}

fn can_plant_bean() {
    is_child && (MagicBean, 10)
}

fn has_explosives() {
    BombBag
}
