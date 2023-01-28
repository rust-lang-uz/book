fn main() {
    // ANCHOR: here
    let mut s = String::from("salom");

    let r1 = &s; // muammo yo'q
    let r2 = &s; // muammo yo'q
    let r3 = &mut s; // KATTA MUAMMO

    println!("{}, {}, va {}", r1, r2, r3);
    // ANCHOR_END: here
}
