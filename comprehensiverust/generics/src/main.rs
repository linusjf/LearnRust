#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2D<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{integer:?} and {float:?}");
    println!("{0:?} and {1:?}", integer.x, float.y);
    let integer = Point2D { x: 5.0, y: 10 };
    let float = Point2D { x: 1, y: 4.0 };
    println!("{integer:?} and {float:?}");
    println!("{0:?} and {1:?}", integer.x, float.y);
}
