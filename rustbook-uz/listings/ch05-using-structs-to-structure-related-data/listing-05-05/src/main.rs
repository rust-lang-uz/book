struct Foydalanuvchi {
    faollik: bool,
    foydalanuvchi: String,
    email: String,
    kirish_hisobi: u64,
}

// ANCHOR: here
fn foydalanuvchi_yaratish(email: String, foydalanuvchi: String) -> Foydalanuvchi {
    Foydalanuvchi {
        faollik: true,
        foydalanuvchi,
        email,
        kirish_hisobi: 1,
    }
}
// ANCHOR_END: here

fn main() {
    let foydalanuvchi1 = foydalanuvchi_yaratish(
        String::from("ismoilovdev@example.com"),
        String::from("ismoilovdev"),
    );
}
