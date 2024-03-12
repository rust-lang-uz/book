struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("CustomSmartPointerni `{}` ma'lumot bilan Drop qilish!", self.data);
    }
}

// ANCHOR: here
fn main() {
    let c = CustomSmartPointer {
        data: String::from("ma'lumot"),
    };
    println!("CustomSmartPointer yaratildi.");
    drop(c);
    println!("main tugashidan oldin CustomSmartPointer drop qilindi.");
}
// ANCHOR_END: here
