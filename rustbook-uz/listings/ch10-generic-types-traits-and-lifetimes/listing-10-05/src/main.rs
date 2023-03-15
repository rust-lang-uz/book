fn eng_katta<T>(list: &[T]) -> &T {
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

    let char_list = vec!['y', 'm', 'a', 'q'];

    let natija = eng_katta(&char_list);
    println!("Eng katta belgi {}", natija);
}
