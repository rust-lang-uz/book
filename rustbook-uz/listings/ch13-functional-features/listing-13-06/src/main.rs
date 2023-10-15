use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Closureni aniqlashdan oldin: {:?}", list);

    thread::spawn(move || println!("{:?} threaddan", list))
        .join()
        .unwrap();
}
