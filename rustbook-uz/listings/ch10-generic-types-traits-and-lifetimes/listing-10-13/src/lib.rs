pub trait Xulosa {
    fn umumiy_xulosa(&self) -> String;
}

// ANCHOR: here
pub struct YangiMaqola {
    pub sarlavha: String,
    pub manzil: String,
    pub muallif: String,
    pub mazmuni: String,
}

impl Xulosa for YangiMaqola {
    fn umumiy_xulosa(&self) -> String {
        format!("{}, by {} ({})", self.sarlavha, self.muallif, self.manzil)
    }
}

pub struct Maqola {
    pub foydalanuvchi: String,
    pub mazmuni: String,
    pub javob_berish: bool,
    pub repost: bool,
}

impl Xulosa for Maqola {
    fn umumiy_xulosa(&self) -> String {
        format!("{}: {}", self.foydalanuvchi, self.mazmuni)
    }
}
// ANCHOR_END: here
