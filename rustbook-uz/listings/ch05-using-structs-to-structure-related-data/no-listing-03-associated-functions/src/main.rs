#[derive(Debug)]
struct Kvadrat {
    kenglik: u32,
    balandlik: u32,
}

// ANCHOR: here
impl Kvadrat {
    fn kvadrat(size: u32) -> Self {
        Self {
            kenglik: size,
            balandlik: size,
        }
    }
}
// ANCHOR_END: here

fn main() {
    let kv = Kvadrat::kvadrat(3);
}
