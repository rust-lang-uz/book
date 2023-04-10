// ANCHOR: here
pub fn ikkita_qoshish(a: i32) -> i32 {
    a + 3
}
// ANCHOR_END: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ikkita_qosh() {
        assert_eq!(4, ikkita_qoshish(2));
    }
}
