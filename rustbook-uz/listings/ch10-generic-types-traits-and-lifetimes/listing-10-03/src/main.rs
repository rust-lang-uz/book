// ANCHOR: here
fn eng_katta(list: &[i32]) -> &i32 {
    let mut eng_katta = &list[0];

    for element in list {
        if element > eng_katta {
            eng_katta = element;
        }
    }

    eng_katta
}

fn main() {
    let raqamlar_listi = vec![34, 50, 25, 100, 65];

    let natija = eng_katta(&raqamlar_listi);
    println!("Eng katta raqam {}", natija);
    // ANCHOR_END: here
    assert_eq!(*natija, 100);
    // ANCHOR: here

    let raqamlar_listi = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let natija = eng_katta(&raqamlar_listi);
    println!("Eng katta raqam {}", natija);
    // ANCHOR_END: here
    assert_eq!(*natija, 6000);
    // ANCHOR: here
}
// ANCHOR_END: here
