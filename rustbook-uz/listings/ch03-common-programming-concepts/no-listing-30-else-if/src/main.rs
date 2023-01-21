fn main() {
    let raqam = 6;

    if raqam % 4 == 0 {
        println!("raqam 4 ga bo'linadi");
    } else if raqam % 3 == 0 {
        println!("raqam 3 ga bo'linadi");
    } else if raqam % 2 == 0 {
        println!("raqam 2 ga bo'linadi");
    } else {
        println!("raqam 4, 3 yoki 2 ga bo'linmaydi");
    }
}
