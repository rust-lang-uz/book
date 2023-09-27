fn main() {
    let list = vec![1, 2, 3];
    println!("Closureni belgilashdan oldin: {:?}", list);

    let faqat_borrow = || println!("Closuredan: {:?}", list);

    println!("Closureni chaqirishdan oldin: {:?}", list);
    faqat_borrow();
    println!("Chaqirilgandan keyin closure: {:?}", list);
}
