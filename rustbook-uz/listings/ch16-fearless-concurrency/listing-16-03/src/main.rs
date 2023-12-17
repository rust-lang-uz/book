use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Mana vektor: {:?}", v);
    });

    handle.join().unwrap();
}
