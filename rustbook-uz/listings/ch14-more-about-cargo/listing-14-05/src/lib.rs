// ANCHOR: here
//! # Rassom
//!
//! Badiiy tushunchalarni modellashtirish uchun kutubxona.

pub use self::turlar::AsosiyRang;
pub use self::turlar::IkkilamchiRang;
pub use self::yordamchi::aralashtirish;

pub mod turlar {
    // --snip--
    // ANCHOR_END: here
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
    // ANCHOR: here
}

pub mod yordamchi {
    // --snip--
    // ANCHOR_END: here
    use crate::turlar::*;

    /// Ikkilamchi rang yaratish uchun ikkita asosiy rangni teng
    /// miqdorda birlashtiradi.
    pub fn aralashtirish(c1: AsosiyRang, c2: AsosiyRang) -> IkkilamchiRang {
        IkkilamchiRang::Qovoqrang
    }
    // ANCHOR: here
}
// ANCHOR_END: here
