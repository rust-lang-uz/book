fn main() {
    // ANCHOR: here
    let narda_toshi = 9;
    match narda_toshi {
        3 => chiroyli_shlyapa_qoshish(),
        7 => chiroyli_shlyapani_ochirish(),
        _ => (),
    }

    fn chiroyli_shlyapa_qoshish() {}
    fn chiroyli_shlyapani_ochirish() {}
    // ANCHOR_END: here
}
