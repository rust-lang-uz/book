// ANCHOR: here
fn birinchi_soz(s: &String) -> usize {
    // ANCHOR: as_bytes
    let bayt = s.as_bytes();
    // ANCHOR_END: as_bytes

    // ANCHOR: iter
    for (i, &item) in bayt.iter().enumerate() {
        // ANCHOR_END: iter
        // ANCHOR: inside_for
        if item == b' ' {
            return i;
        }
    }

    s.len()
    // ANCHOR_END: inside_for
}
// ANCHOR_END: here

fn main() {}
