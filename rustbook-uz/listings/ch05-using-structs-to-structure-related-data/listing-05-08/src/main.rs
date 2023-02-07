// ANCHOR: all
fn main() {
    let kenglik1 = 30;
    let balandlik1 = 50;

    println!(
        "To'rtburchakning maydoni {} kvadrat piksel.",
        area(kenglik1, balandlik1)
    );
}

// ANCHOR: here
fn area(kenglik: u32, balandlik: u32) -> u32 {
    // ANCHOR_END: here
    kenglik * balandlik
}
// ANCHOR_END: all
