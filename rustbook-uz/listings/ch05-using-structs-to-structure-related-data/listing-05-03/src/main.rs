struct Foydalanuvchi {
    faollik: bool,
    foydalanuvchi: String,
    email: String,
    kirish_hisobi: u64,
}

// ANCHOR: here
fn main() {
    let foydalanuvchi1 = Foydalanuvchi {
        faollik: true,
        foydalanuvchi: String::from("ismoilovdev"),
        email: String::from("ismoilovdev@example.com"),
        kirish_hisobi: 1,
    };

    foydalanuvchi1.email = String::from("asilbek@example.com");
}
// ANCHOR_END: here
