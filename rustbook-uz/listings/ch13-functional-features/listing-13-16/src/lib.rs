#[derive(PartialEq, Debug)]
struct Poyabzal {
    olchami: u32,
    uslub: String,
}

fn olcham_boyicha_poyabzal(poyabzal: Vec<Poyabzal>, poyabzal_olchami: u32) -> Vec<Poyabzal> {
    poyabzal.into_iter().filter(|s| s.size == poyabzal_olchami).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn olcham_boyicha_filterlash() {
        let poyabzal = vec![
            Poyabzal {
                olchami: 10,
                uslub: String::from("krossovka"),
            },
            Poyabzal {
                olchami: 13,
                uslub: String::from("sandal"),
            },
            Poyabzal {
                olchami: 10,
                uslub: String::from("etik"),
            },
        ];

        let in_my_size = olcham_boyicha_poyabzal(poyabzal, 10);

        assert_eq!(
            in_my_size,
            vec![
                Poyabzal {
                    olchami: 10,
                    uslub: String::from("krossovka")
                },
                Poyabzal {
                    olchami: 10,
                    uslub: String::from("etik")
                },
            ]
        );
    }
}
