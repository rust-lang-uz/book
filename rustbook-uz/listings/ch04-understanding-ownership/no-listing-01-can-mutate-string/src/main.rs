fn main() {
    // ANCHOR: here
    let mut s = String::from("salom");

    s.push_str(", rust!"); // push_str() satrga literal qo'shadi

    println!("{}", s); // Bu “salom, rust!” deb chop etiladi
                       // ANCHOR_END: here
}
