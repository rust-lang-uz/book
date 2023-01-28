fn main() {
    // ANCHOR: here
    let mut s = String::from("salom");

    let r1 = &s; // muammo yo'q
    let r2 = &s; // muammo yo'q
    println!("{} va {}", r1, r2);
    // r1 va r2 o'zgaruvchilari bu nuqtadan keyin ishlatilmaydi

    let r3 = &mut s; // muammo yo'q
    println!("{}", r3);
    // ANCHOR_END: here
}
