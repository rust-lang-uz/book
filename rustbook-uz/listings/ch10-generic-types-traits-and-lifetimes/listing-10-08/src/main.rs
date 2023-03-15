struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let ikkita_integer = Point { x: 5, y: 10 };
    let ikkita_float = Point { x: 1.0, y: 4.0 };
    let integer_va_float = Point { x: 5, y: 4.0 };
}
