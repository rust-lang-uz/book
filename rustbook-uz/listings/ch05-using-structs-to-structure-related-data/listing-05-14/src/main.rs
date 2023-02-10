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
