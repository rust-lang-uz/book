// ANCHOR: here
pub fn salomlashish(name: &str) -> String {
    String::from("Salom!")
}
// ANCHOR_END: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn salomlash() {
        let natija = salomlashish("Azizbek");
        assert!(natija.contains("Azizbek"));
    }
}
