use std::io;

fn main() {
    println!("Raqamni topish o'yini!");

    println!("Iltimos, taxminingizni kiriting.");

    let mut taxmin = String::new();

    io::stdin()
        .read_line(&mut taxmin)
        .expect("Satrni o‘qib bo‘lmadi");

    println!("Sizning taxminingiz: {taxmin}");
}
