mod uyning_orqasi {
    pub struct Nonushta {
        pub yopilgan_non: String,
        mavsumiy_meva: String,
    }

    impl Nonushta {
        pub fn yoz(yopilgan_non: &str) -> Nonushta {
            Nonushta {
                yopilgan_non: String::from(yopilgan_non),
                mavsumiy_meva: String::from("shaftoli"),
            }
        }
    }
}

pub fn restoranda_ovqatlanish() {
    // Yozda javdar yopilgan noni bilan nonushta buyurtma qiling
    let mut ovqat = uyning_orqasi::Nonushta::yoz("Javdar");
    // Qaysi nonni xohlashimiz haqidagi fikrimizni o'zgartiring
    ovqat.yopilgan_non = String::from("Bug'doy");
    println!("Iltimos, {}li yopilgan nonni istayman", ovqat.yopilgan_non);

    // Agar izohni olib tashlasak, keyingi qator kompilyatsiya qilinmaydi;
    // ovqat bilan birga keladigan mavsumiy mevalarni ko'rish yoki 
    // o'zgartirishga ruxsat berilmagan
    // ovqat.mavsumiy_meva = String::from("ko'katlar");
}
