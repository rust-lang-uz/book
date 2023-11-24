//! # Rassom
//!
//! Badiiy tushunchalarni modellashtirish uchun kutubxona.

pub use self::turlar::AsosiyRang;
pub use self::turlar::IkkilamchiRang;
pub use self::yordamchi::yordamchi;

pub mod turlar {
    /// RYB rang modeliga muvofiq asosiy ranglar.
    pub enum AsosiyRang {
        Qizil,
        Sariq,
        Kok,
    }

    /// RYB rang modeliga muvofiq ikkinchi darajali ranglar.
    pub enum IkkilamchiRang {
        Qovoqrang,
        Yashil,
        Siyohrang,
    }
}

pub mod yordamchi {
    use crate::turlar::*;

    /// Ikkilamchi rang yaratish uchun ikkita asosiy rangni teng
    /// miqdorda birlashtiradi.
    pub fn yordamchi(c1: AsosiyRang, c2: AsosiyRang) -> IkkilamchiRang {
        IkkilamchiRang::Qovoqrang
    }
}
