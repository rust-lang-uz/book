fn main() {
    let s1 = String::from("salom");

    let len = uzunlikni_hisoblash(&s1);

    println!("'{}' uzunligi {}.", s1, len);
}

// ANCHOR: here
fn uzunlikni_hisoblash(s: &String) -> usize { // s - Stringga reference(havola)
    s.len()
} // Bu yerda s scopedan chiqib ketadi. Lekin u nazarda tutgan itemga ownership qilmagani
  // uchun u tashlanmaydi.
// ANCHOR_END: here
