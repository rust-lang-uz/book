// ANCHOR: here
//! # Rassom
//!
//! Badiiy tushunchalarni modellashtirish uchun kutubxona.

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
    pub fn aralashtirish(c1: AsosiyRang, c2: AsosiyRang) -> IkkilamchiRang {
        // --snip--
        // ANCHOR_END: here
        unimplemented!();
        // ANCHOR: here
    }
}
// ANCHOR_END: here
