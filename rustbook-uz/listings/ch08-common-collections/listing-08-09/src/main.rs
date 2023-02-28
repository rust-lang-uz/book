fn main() {
    // ANCHOR: here
    enum ElektronJadval {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let qator = vec![
        ElektronJadval::Int(3),
        ElektronJadval::Text(String::from("ko'k")),
        ElektronJadval::Float(10.12),
    ];
    // ANCHOR_END: here
}
