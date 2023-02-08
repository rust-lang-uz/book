#[derive(Debug)]
struct Kvadrat {
    kenglik: u32,
    balandlik: u32,
}
fn main() {
    let kvadrat1 = Kvadrat {
        kenglik: 30,
        balandlik: 50,
    };

    println!("kvadrat1 - {}", kvadrat1);
}
