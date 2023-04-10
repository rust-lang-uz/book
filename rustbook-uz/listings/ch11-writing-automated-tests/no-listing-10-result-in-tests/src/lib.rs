#[cfg(test)]
mod tests {
    #[test]
    fn ishlaydi() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("ikki qo'shish ikki to'rtga teng emas"))
        }
    }
}
