fn main() {
    let mut hisoblagich = 0;

    let natija = loop {
        hisoblagich += 1;

        if hisoblagich == 10 {
            break hisoblagich * 2;
        }
    };

    println!("Natija: {natija}");
}
