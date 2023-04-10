pub struct Taxmin {
    qiymat: i32,
}

impl Taxmin  {
    pub fn new(qiymat: i32) -> Taxmin  {
        // ANCHOR: here
        if qiymat < 1 {
            panic!(
                "Taxmin qilingan qiymat 1 dan 100 gacha bo'lishi kerak, {} qabul qilinmaydi.",
                qiymat
            );
        } else if qiymat > 100 {
            panic!(
                "Taxmin qilingan qiymat 1 dan katta yoki teng bo'lishi kerak, {} qabul qilinmaydi.",
                qiymat
            );
        }
        // ANCHOR_END: here

        Taxmin  { qiymat }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "100 dan kichik yoki teng")]
    fn _100_dan_ortiq() {
        Taxmin ::new(200);
    }
}
