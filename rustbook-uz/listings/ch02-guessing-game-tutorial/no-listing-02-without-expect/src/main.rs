use std::io;

fn main() {
    println!("Raqamni topish o'yini!");

    println!("Iltimos, taxminingizni kiriting.");

    let mut taxmin = String::new();

    io::stdin().read_line(&mut taxmin);

    println!("Sizning taxminingiz: {taxmin}");
}