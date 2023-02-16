fn main() {
    // ANCHOR: here
    let narda_toshi = 9;
    match narda_toshi {
        3 => chiroyli_shlyapa_qoshish(),
        7 => chiroyli_shlyapani_ochirish(),
        boshqa => player_harakati(boshqa),
    }

    fn chiroyli_shlyapa_qoshish() {}
    fn chiroyli_shlyapani_ochirish() {}
    fn player_harakati(bosh_joylar: u8) {}
    // ANCHOR_END: here
}
