#[derive(Debug)]
struct Kvadrat {
    kenglik: u32,
    balandlik: u32,
}

// ANCHOR: here
impl Kvadrat {
    fn area(&self) -> u32 {
        self.kenglik * self.balandlik
    }
}

impl Kvadrat {
    fn ushlab_tur(&self, other: &Kvadrat) -> bool {
        self.kenglik > other.kenglik && self.balandlik > other.balandlik
    }
}
// ANCHOR_END: here

fn main() {
    let kvadrat1 = Kvadrat {
        kenglik: 30,
        balandlik: 50,
    };
    let kvadrat2 = Kvadrat {
        kenglik: 10,
        balandlik: 40,
    };
    let kvadrat3 = Kvadrat {
        kenglik: 60,
        balandlik: 45,
    };

    println!("kvadrat1 kvadrat2ni ushlab turadimi? {}", kvadrat1.ushlab_tur(&kvadrat2));
    println!("kvadrat1 kvadrat3ni ushlab turadimi? {}", kvadrat1.ushlab_tur(&kvadrat3));
}
