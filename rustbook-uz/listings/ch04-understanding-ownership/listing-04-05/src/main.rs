fn main() {
    let s1 = String::from("salom");

    let (s2, len) = uzunlikni_hisoblash(s1);

    println!("'{}' uzunligi {}.", s2, len);
}

fn uzunlikni_hisoblash(s: String) -> (String, usize) {
    let length = s.len(); // len() string uzunligini qaytaradi

    (s, length)
}
