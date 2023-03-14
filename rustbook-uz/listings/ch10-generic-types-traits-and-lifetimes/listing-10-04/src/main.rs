// ANCHOR: here
fn eng_katta_i32(list: &[i32]) -> &i32 {
    let mut eng_katta = &list[0];

    for element in list {
        if element > eng_katta {
            eng_katta = element;
        }
    }

    eng_katta
}

fn eng_katta_char(list: &[char]) -> &char {
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

    let natija = eng_katta_i32(&raqamlar_listi);
    println!("Eng katta raqam {}", natija);
    // ANCHOR_END: here
    assert_eq!(*natija, 100);
    // ANCHOR: here

    let char_list = vec!['y', 'm', 'a', 'q'];

    let natija = eng_katta_char(&char_list);
    println!("Eng katta belgi {}", natija);
    // ANCHOR_END: here
    assert_eq!(*natija, 'y');
    // ANCHOR: here
}
// ANCHOR_END: here
