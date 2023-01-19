use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Raqamni topish o'yini!");

    let yashirin_raqam = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Iltimos, taxminingizni kiriting.");

        let mut taxmin = String::new();

        io::stdin()
            .read_line(&mut taxmin)
            .expect("Satrni o‘qib bo‘lmadi");

        let taxmin: u32 = match taxmin.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Sizning taxminingiz: {taxmin}");

        match taxmin.cmp(&yashirin_raqam) {
            Ordering::Less => println!("Raqam Kichik!"),
            Ordering::Greater => println!("Raqam katta!"),
            Ordering::Equal => {
                println!("Siz yutdingiz!");
                break;
            }
        }
    }
}
