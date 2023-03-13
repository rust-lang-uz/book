fn main() {
    let raqamlar_listi = vec![34, 50, 25, 100, 65];

    let mut eng_katta = &raqamlar_listi[0];

    for raqam in &raqamlar_listi {
        if raqam > eng_katta {
            eng_katta = raqam;
        }
    }

    println!("Eng katta raqam {}", eng_katta);

    let raqamlar_listi = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut eng_katta = &raqamlar_listi[0];

    for raqam in &raqamlar_listi {
        if raqam > eng_katta {
            eng_katta = raqam;
        }
    }

    println!("Eng katta raqam {}", eng_katta);
}
