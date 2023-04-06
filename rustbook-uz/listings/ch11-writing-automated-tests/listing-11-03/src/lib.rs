// ANCHOR: here
#[cfg(test)]
mod tests {
    #[test]
    fn tadqiqot() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn boshqa() {
        panic!("Ushbu test muvaffaqiyatsizlikka uchradil");
    }
}
// ANCHOR_END: here
