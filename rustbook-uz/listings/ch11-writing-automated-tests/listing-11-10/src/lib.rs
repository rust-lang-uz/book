fn print_qiladi_va_10_qaytaradi(a: i32) -> i32 {
    println!("Men {} qiymatini oldim", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_muvaffaqiyatli() {
        let qiymat = print_qiladi_va_10_qaytaradi(4);
        assert_eq!(10, qiymat);
    }

    #[test]
    fn test_muvaffaqiyatsiz() {
        let qiymat = print_qiladi_va_10_qaytaradi(8);
        assert_eq!(5, qiymat);
    }
}
