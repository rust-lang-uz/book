fn main() {
    // ANCHOR: here
    let mut v = vec![1, 2, 3, 4, 5];

    let birinchi = &v[0];

    v.push(6);

    println!("Birinchi element: {birinchi}");
    // ANCHOR_END: here
}
