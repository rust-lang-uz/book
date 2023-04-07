// ANCHOR: here
#[derive(Debug)]
struct Kvadrat {
    kenglik: u32,
    balandlik: u32,
}

impl Kvadrat {
    fn ushlab_tur(&self, boshqa: &Kvadrat) -> bool {
        self.kenglik > other.kenglik && self.balandlik > boshqa.balandlik
    }
}
// ANCHOR_END: here
