use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("salom");
        tx.send(val).unwrap();
        println!("qandaysan {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Tushundim: {}", received);
}
