fn main() {
    let mut list = vec![1, 2, 3];
    println!("Closureni aniqlashdan oldin: {:?}", list);

    let mut ozgaruvchan_borrow = || list.push(7);

    ozgaruvchan_borrow();
    println!("Chaqirilgandan keyin closure: {:?}", list);
}
