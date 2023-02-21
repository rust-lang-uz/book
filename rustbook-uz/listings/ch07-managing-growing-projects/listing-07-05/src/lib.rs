mod uyning_oldi {
    pub mod xizmat {
        fn navbat_listiga_qoshish() {}
    }
}

pub fn restoranda_ovqatlanish() {
    // Mutlaq yo'l (Absolute path)
    crate::uyning_oldi::xizmat::navbat_listiga_qoshish();

    // Nisbiy yo'l (Relative path)
    uyning_oldi::xizmat::navbat_listiga_qoshish();
}
