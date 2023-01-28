// ANCHOR: all
fn main() {
    // ANCHOR: here
    let s1 = String::from("salom");

    let len = uzunlikni_hisoblash(&s1);
    // ANCHOR_END: here

    println!("'{}' uzunligi {}.", s1, len);
}

fn uzunlikni_hisoblash(s: &String) -> usize {
    s.len()
}
// ANCHOR_END: all
