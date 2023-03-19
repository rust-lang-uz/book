use aggregator::{self, Xulosa, Maqola};

fn main() {
    // ANCHOR: here
    let maqola = Maqola {
        foydalanuvchi: String::from("ismoilovdev"),
        mazmuni: String::from(
            "Rust kitobi juda foydali ekan, men juda ko'p bilimlarni o'zlashtirdim",
        ),
        javob_berish: false,
        repost: false,
    };

    println!("1 ta yangi xabar: {}", maqola.umumiy_xulosa());
    // ANCHOR_END: here
}
