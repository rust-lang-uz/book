fn main() {
    // ANCHOR: here
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        tur: IpAddrKind,
        manzil: String,
    }

    let asosiy = IpAddr {
        tur: IpAddrKind::V4,
        manzil: String::from("127.0.0.1"),
    };

    let orqaga_qaytish = IpAddr {
        tur: IpAddrKind::V6,
        manzil: String::from("::1"),
    };
    // ANCHOR_END: here
}
