pub fn ikkita_qosh(a: i32) -> i32 {
    ichki_qoshuvchi(a, 2)
}

fn ichki_qoshuvchi(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ichki() {
        assert_eq!(4, ichki_qoshuvchi(2, 2));
    }
}
