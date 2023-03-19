// ANCHOR: here
pub trait Xulosa {
    fn muallif_haqida(&self) -> String;

    fn umumiy_xulosa(&self) -> String {
        format!("(Batafsil: {}...)", self.muallif_haqida())
    }
}
// ANCHOR_END: here

pub struct Maqola {
    pub foydalanuvchi: String,
    pub mazmuni: String,
    pub javob_berish: bool,
    pub repost: bool,
}

// ANCHOR: impl
impl Xulosa for Maqola {
    fn muallif_haqida(&self) -> String {
        format!("@{}", self.foydalanuvchi)
    }
}
// ANCHOR_END: impl
