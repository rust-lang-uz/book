#[derive(Debug)]
struct Kvadrat {
    kenglik: u32,
    balandlik: u32,
}

// ANCHOR: here
impl Kvadrat {
    fn kenglik(&self) -> bool {
        self.kenglik > 0
    }
}

fn main() {
    let kvadrat1 = Kvadrat {
        kenglik: 30,
        balandlik: 50,
    };

    if kvadrat1.kenglik() {
        println!("To'rtburchakning kengligi nolga teng bo'lmagan; bu {}", kvadrat1.kenglik);
    }
}
// ANCHOR_END: here
