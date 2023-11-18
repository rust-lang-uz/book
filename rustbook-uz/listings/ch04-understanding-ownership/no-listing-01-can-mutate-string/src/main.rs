fn main() {
    // ANCHOR: here
    let mut s = String::from("salom");

    s.push_str(", dunyo!"); // push_str() satrga literal qo'shadi

    println!("{}", s); // Bu “salom, dunyo!” deb chop etiladi
                       // ANCHOR_END: here
}
