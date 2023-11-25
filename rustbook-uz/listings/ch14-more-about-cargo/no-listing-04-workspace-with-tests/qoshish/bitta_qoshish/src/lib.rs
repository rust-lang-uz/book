pub fn bitta_qoshish(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ishlamoqda() {
        assert_eq!(3, bitta_qoshish(2));
    }
}
