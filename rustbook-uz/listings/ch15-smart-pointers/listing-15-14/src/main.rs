struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("CustomSmartPointerni `{}` ma'lumot bilan tashlab yuborish!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("menga tegishli"),
    };
    let d = CustomSmartPointer {
        data: String::from("boshqaga tegishli"),
    };
    println!("CustomSmartPointerlar yaratildi.");
}
