#[derive(Debug, PartialEq, Copy, Clone)]
enum FutbolkaRangi {
    Qizil,
    Moviy,
}

struct Inventarizatsiya {
    futbolkalar: Vec<FutbolkaRangi>,
}

impl Inventarizatsiya {
    fn yutuq(&self, user_preference: Option<FutbolkaRangi>) -> FutbolkaRangi {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> FutbolkaRangi {
        let mut qizil_raqam = 0;
        let mut moviy_raqam = 0;

        for rang in &self.futbolkalar {
            match rang {
                FutbolkaRangi::Qizil => qizil_raqam += 1,
                FutbolkaRangi::Moviy => moviy_raqam += 1,
            }
        }
        if qizil_raqam > moviy_raqam {
            FutbolkaRangi::Qizil
        } else {
            FutbolkaRangi::Moviy
        }
    }
}

fn main() {
    let store = Inventarizatsiya {
        futbolkalar: vec![FutbolkaRangi::Moviy, FutbolkaRangi::Qizil, FutbolkaRangi::Moviy],
    };

    let user_pref1 = Some(FutbolkaRangi::Qizil);
    let yutuq1 = store.yutuq(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, yutuq1
    );

    let user_pref2 = None;
    let yutuq2 = store.yutuq(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, yutuq2
    );
}