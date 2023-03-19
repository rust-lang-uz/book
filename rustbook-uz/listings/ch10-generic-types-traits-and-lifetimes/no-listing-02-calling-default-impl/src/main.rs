use aggregator::{self, YangiMaqola, Xulosa};

fn main() {
    // ANCHOR: here
    let maqola = YangiMaqola {
        sarlavha: String::from("Tesla yangi elektromobil ustida ishlayapti"),
        manzil: String::from("USA"),
        muallif: String::from("Elon Musk"),
        mazmuni: String::from(
            "Hozirgi kunda Tesla yangi innovatsion elektromobil\
             ustida ishlamoqda.",
        ),
    };

    println!("Yangi maqola mavjud! {}", maqola.umumiy_xulosa());
    // ANCHOR_END: here
}
