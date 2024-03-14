use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn salom(nom: &str) {
    println!("Salom, {nom}!");
}

// ANCHOR: here
fn main() {
    let m = MyBox::new(String::from("Rust"));
    salom(&(*m)[..]);
}
// ANCHOR_END: here
