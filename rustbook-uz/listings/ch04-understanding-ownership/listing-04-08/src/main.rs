fn birinchi_soz(s: &String) -> usize {
    let baytlar = s.as_bytes();

    for (i, &item) in baytlar.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// ANCHOR: here
fn main() {
    let mut s = String::from("hello world");

    let soz = birinchi_soz(&s); // soz 5 qiymatini oladi

    s.clear(); // bu Stringni bo'shatib, uni "" ga tenglashtiradi

    // soz hali ham bu erda 5 qiymatiga ega, ammo biz 5 qiymatini meaningfull ishlatishimiz
    // mumkin bo'lgan boshqa qator yo'q. soz endi mutlaqo yaroqsiz!
}
// ANCHOR_END: here
