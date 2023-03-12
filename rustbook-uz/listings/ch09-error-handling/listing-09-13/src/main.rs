use rand::Rng;
use std::cmp::Ordering;
use std::io;

// ANCHOR: here
pub struct Taxmin {
    qiymat: i32,
}

impl Taxmin {
    pub fn new(qiymat: i32) -> Taxmin {
        if qiymat < 1 || qiymat > 100 {
            panic!("Taxmin qilingan qiymat 1 dan 100 gacha bo'lishi kerak, {} qabul qilnmaydi.", qiymat);
        }

        Taxmin { qiymat }
    }

    pub fn qiymat(&self) -> i32 {
        self.qiymat
    }
}
// ANCHOR_END: here

fn main() {
    println!("Raqamni topish o'yini!");

    let yashirin_raqam = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Iltimos, taxminingizni kiriting.");

        let mut taxmin = String::new();

        io::stdin()
            .read_line(&mut taxmin)
            .expect("Satrni o‘qib bo‘lmadi");

        let taxmin: i32 = match taxmin.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let taxmin = Taxmin::new(taxmin);

        match taxmin.qiymat().cmp(&yashirin_raqam) {
            Ordering::Less => println!("Raqam Kichik!"),
            Ordering::Greater => println!("Raqam katta!"),
            Ordering::Equal => {
                println!("Siz yutdingiz!");
                break;
            }
        }
    }
}
