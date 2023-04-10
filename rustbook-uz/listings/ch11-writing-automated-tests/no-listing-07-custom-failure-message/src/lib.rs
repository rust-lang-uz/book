pub fn salomlashish(name: &str) -> String {
    String::from("Salom!")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ANCHOR: here
    #[test]
    fn salomlash() {
        let natija = salomlashish("Azizbek");
        assert!(
            natija.contains("Azizbek"),
            "Salomlashishda ism yo'q, qiymat `{}` edi",
            natija
        );
    }
    // ANCHOR_END: here
}
