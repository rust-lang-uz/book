// ANCHOR: here
fn main() {
    let raqamlar_listi = vec![34, 50, 25, 100, 65];

    let mut eng_katta = &raqamlar_listi[0];

    for raqam in &raqamlar_listi {
        if raqam > eng_katta {
            eng_katta = raqam;
        }
    }

    println!("Eng katta raqam {}", eng_katta);
    // ANCHOR_END: here
    assert_eq!(*eng_katta, 100);
    // ANCHOR: here
}
// ANCHOR_END: here
