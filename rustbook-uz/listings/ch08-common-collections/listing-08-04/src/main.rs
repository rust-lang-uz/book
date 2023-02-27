fn main() {
    // ANCHOR: here
    let v = vec![1, 2, 3, 4, 5];

    let uchinchi: &i32 = &v[2];
    println!("Uchinchi element {uchinchi}");

    let uchinchi: Option<&i32> = v.get(2);
    match uchinchi {
        Some(uchinchi) => println!("Uchinchi element {uchinchi}"),
        None => println!("Uchinchi element yo'q."),
    }
    // ANCHOR_END: here
}
