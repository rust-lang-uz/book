use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Iltimos, array indeksini kiriting.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Satrni o‘qib bo‘lmadi");

    let index: usize = index
        .trim()
        .parse()
        .expect("Kiritilgan indeks raqam emas");

    let element = a[index];

    println!("{index} indeksidagi elementning qiymati: {element}");
}
