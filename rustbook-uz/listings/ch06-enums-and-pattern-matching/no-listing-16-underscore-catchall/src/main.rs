fn main() {
    // ANCHOR: here
    let narda_toshi = 9;
    match narda_toshi {
        3 => chiroyli_shlyapa_qoshish(),
        7 => chiroyli_shlyapani_ochirish(),
        _ => qaytadan(),
    }

    fn chiroyli_shlyapa_qoshish() {}
    fn chiroyli_shlyapani_ochirish() {}
    fn qaytadan() {}
    // ANCHOR_END: here
}
