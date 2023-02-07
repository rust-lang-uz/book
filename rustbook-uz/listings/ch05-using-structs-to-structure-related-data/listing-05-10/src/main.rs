struct Kvadrat {
    kenglik: u32,
    balandlik: u32,
}

fn main() {
    let kvadrat1 = Kvadrat {
        kenglik: 30,
        balandlik: 50,
    };

    println!(
        "To'rtburchakning maydoni {} kvadrat pikselga teng.",
        area(&kvadrat1)
    );
}

fn area(kvadrat: &Kvadrat) -> u32 {
    kvadrat.kenglik * kvadrat.balandlik
}
