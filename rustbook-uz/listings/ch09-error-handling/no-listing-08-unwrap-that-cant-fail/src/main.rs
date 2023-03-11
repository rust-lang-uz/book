fn main() {
    // ANCHOR: here
    use std::net::IpAddr;

    let asosiy: IpAddr = "127.0.0.1"
        .parse()
        .expect("Qattiq kodlangan IP manzil haqiqiy bo'lishi kerak");
    // ANCHOR_END: here
}
