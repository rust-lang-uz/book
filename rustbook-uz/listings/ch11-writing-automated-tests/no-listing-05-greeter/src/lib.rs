pub fn salomlashish(name: &str) -> String {
    format!("Salom {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn salomlash() {
        let natija = salomlashish("Azizbek");
        assert!(natija.contains("Azizbek"));
    }
}
