use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Raqamni topish o'yini!");

    let yashirin_raqam = rand::thread_rng().gen_range(1..=100);

    // ANCHOR: here
    // --snip--

    println!("Yashirin raqam: {yashirin_raqam}");

    loop {
        println!("Iltimos, taxminingizni kiriting.");

        // --snip--

        // ANCHOR_END: here

        let mut taxmin = String::new();

        io::stdin()
            .read_line(&mut taxmin)
            .expect("Satrni o‘qib bo‘lmadi");

        let taxmin: u32 = taxmin.trim().parse().expect("Iltimos, raqam yozing!");

        println!("Sizning taxminingiz: {taxmin}");

        // ANCHOR: here
        match taxmin.cmp(&yashirin_raqam) {
            Ordering::Less => println!("Raqam Kichik!"),
            Ordering::Greater => println!("Raqam katta!"),
            Ordering::Equal => println!("Siz yutdingiz!"),
        }
    }
}
// ANCHOR_END: here
