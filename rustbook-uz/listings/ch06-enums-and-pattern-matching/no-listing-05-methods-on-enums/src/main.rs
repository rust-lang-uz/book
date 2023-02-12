fn main() {
    enum Xabar {
        Chiqish,
        Kochirish { x: i32, y: i32 },
        Yozish(String),
        RangTanlash(i32, i32, i32),
    }

    // ANCHOR: here
    impl Xabar {
        fn chaqiruv(&self) {
            // metod tanasi bu yerda aniqlanadi
        }
    }

    let m = Xabar::Yozish(String::from("salom"));
    m.chaqiruv();
    // ANCHOR_END: here
}
