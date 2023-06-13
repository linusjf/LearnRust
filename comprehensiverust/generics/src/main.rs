#[allow(dead_code)]
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct PointG<T>(T, T);

impl<T> PointG<T> {
    fn x(&self) -> &T {
        &self.0 // + 10
    }
    fn y(&self) -> &T {
        &self.1 // + 10
    }

    // fn set_x(&mut self, x: T)
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
    let p = PointG(5, 10);
    println!("p.x = {}", p.x());
    println!("p.y = {}", p.y());
    let integer = Some(5);
    let float = Some(5.0);
    println!("{integer:?} and {float:?}");
    let integer = OptionI32::Some(5);
    let float = OptionF64::Some(5.0);
    println!("{integer:?} and {float:?}");
    let integer = OptionI32::None;
    let float = OptionF64::None;
    println!("{integer:?} and {float:?}");
}

#[derive(Debug)]
enum OptionF64 {
    Some(f64),
    None,
}

#[derive(Debug)]
enum OptionI32 {
    Some(i32),
    None,
}
