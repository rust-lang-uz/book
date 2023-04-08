#[derive(Debug)]
struct Kvadrat {
    kenglik: u32,
    balandlik: u32,
}

impl Kvadrat {
    fn ushlab_tur(&self, boshqa: &Kvadrat) -> bool {
        self.kenglik > boshqa.kenglik && self.balandlik > boshqa.balandlik
    }
}

// ANCHOR: here
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn katta_kichikni_ushlab_turadi() {
        // --snip--
        // ANCHOR_END: here
        let kattaroq = Kvadrat {
            kenglik: 8,
            balandlik: 7,
        };
        let kichikroq = Kvadrat {
            kenglik: 5,
            balandlik: 1,
        };

        assert!(kattaroq.ushlab_tur(&kichikroq));
        // ANCHOR: here
    }

    #[test]
    fn kichik_kattani_ushlolmaydi() {
        let kattaroq = Kvadrat {
            kenglik: 8,
            balandlik: 7,
        };
        let kichikroq = Kvadrat {
            kenglik: 5,
            balandlik: 1,
        };

        assert!(!kichikroq.ushlab_tur(&kattaroq));
    }
}
// ANCHOR_END: here
