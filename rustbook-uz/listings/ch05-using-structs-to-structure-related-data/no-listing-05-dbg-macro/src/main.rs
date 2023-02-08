#[derive(Debug)]
struct Kvadrat {
    kenglik: u32,
    balandlik: u32,
}
fn main() {
    let masshtab = 2;
    let kvadrat1 = Kvadrat {
        kenglik: dbg!(30 * masshtab),
        balandlik: 50,
    };

    dbg!(&kvadrat1);
}
