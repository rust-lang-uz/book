use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut raqam = m.lock().unwrap();
        *raqam = 6;
    }

    println!("m = {:?}", m);
}
