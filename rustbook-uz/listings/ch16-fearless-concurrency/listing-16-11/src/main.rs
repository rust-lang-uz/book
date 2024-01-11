use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // ANCHOR: here
    // --snip--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("salom"),
            String::from("threaddan"),
            String::from("siz"),
            String::from("uchun"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("ko'plab"),
            String::from("habarlar"),
            String::from("hammasi"),
            String::from("ishlayapti"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Tushundim: {}", received);
    }

    // --snip--
    // ANCHOR_END: here
}
