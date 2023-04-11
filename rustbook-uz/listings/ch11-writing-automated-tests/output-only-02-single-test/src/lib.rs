pub fn ikkita_qoshish(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ikkita_qoshish_va_ikki() {
        assert_eq!(4, ikkita_qoshish(2));
    }

    #[test]
    fn uchta_qoshish_va_ikki() {
        assert_eq!(5, ikkita_qoshish(3));
    }

    #[test]
    fn yuz() {
        assert_eq!(102, ikkita_qoshish(100));
    }
}
