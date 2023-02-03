struct Foydalanuvchi {
    faollik: bool,
    foydalanuvchi: String,
    email: String,
    kirish_hisobi: u64,
}

// ANCHOR: here
fn main() {
    // --snip--
    // ANCHOR_END: here

    let foydalanuvchi1 = Foydalanuvchi {
        email: String::from("ismoilovdev@example.com"),
        foydalanuvchi: String::from("ismoilovdev"),
        faollik: true,
        kirish_hisobi: 1,
    };
    // ANCHOR: here

    let foydalanuvchi2 = Foydalanuvchi {
        email: String::from("asilbek@example.com"),
        ..foydalanuvchi1
    };
}
// ANCHOR_END: here
