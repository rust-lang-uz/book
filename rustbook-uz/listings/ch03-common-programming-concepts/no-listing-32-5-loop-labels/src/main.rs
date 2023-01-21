fn main() {
    let mut hisob = 0;
    'hisoblash: loop {
        println!("hisob = {hisob}");
        let mut qolgan = 10;

        loop {
            println!("qolgan = {qolgan}");
            if qolgan == 9 {
                break;
            }
            if hisob == 2 {
                break 'hisoblash;
            }
            qolgan -= 1;
        }

        hisob += 1;
    }
    println!("Yakuniy hisob = {hisob}");
}
