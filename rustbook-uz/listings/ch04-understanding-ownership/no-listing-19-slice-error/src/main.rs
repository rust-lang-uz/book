fn birinchi_soz(s: &String) -> &str {
    let baytlar = s.as_bytes();

    for (i, &item) in baytlar.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ANCHOR: here
fn main() {
    let mut s = String::from("salom dunyo");

    let soz = birinchi_soz(&s);

    s.clear(); // error!

    println!("birinchi so'z: {}", soz);
}
// ANCHOR_END: here
