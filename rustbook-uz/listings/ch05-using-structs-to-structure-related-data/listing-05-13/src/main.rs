#[derive(Debug)]
struct Kvadrat {
    kenglik: u32,
    balandlik: u32,
}

impl Kvadrat {
    fn area(&self) -> u32 {
        self.kenglik * self.balandlik
    }
}

fn main() {
    let kvadrat1 = Kvadrat {
        kenglik: 30,
        balandlik: 50,
    };

    println!(
        "To'rtburchakning maydoni {} kvadrat pikselga teng.",
        kvadrat1.area()
    );
}

