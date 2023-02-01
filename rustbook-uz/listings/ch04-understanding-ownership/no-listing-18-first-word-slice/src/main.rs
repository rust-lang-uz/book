// ANCHOR: here
fn birinchi_soz(s: &String) -> &str {
    let baytlar = s.as_bytes();

    for (i, &item) in baytlar.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// ANCHOR_END: here

fn main() {}
