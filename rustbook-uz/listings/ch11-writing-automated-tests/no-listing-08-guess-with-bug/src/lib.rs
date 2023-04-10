pub struct Taxmin {
    qiymat: i32,
}

// ANCHOR: here
// --snip--
impl Taxmin {
    pub fn new(qiymat: i32) -> Taxmin {
        if qiymat < 1 {
            panic!("Taxmin qilingan qiymat 1 dan 100 gacha bo'lishi kerak, {} qabul qilinmaydi.", qiymat);
        }

        Taxmin { qiymat }
    }
}
// ANCHOR_END: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn _100_dan_ortiq() {
        Taxmin::new(200);
    }
}
