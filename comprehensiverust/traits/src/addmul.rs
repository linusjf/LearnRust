use auto_ops::*;
#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Add for &Point {
    type Output = Point;

    fn add(self, other: Self) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Add<(i32, i32)> for Point {
    type Output = Point;

    fn add(self, other: (i32, i32)) -> Point {
        Point {
            x: self.x + other.0,
            y: self.y + other.1,
        }
    }
}

impl_op_ex!(-|a: &Point, b: &Point| -> Point {
    Point {
        x: a.x - b.x,
        y: a.y - b.y,
    }
});

pub fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 100, y: 200 };
    let tup = (100, 200);
    println!("{:?} + {:?} = {:?}", p1, p2, p1 + p2);
    println!("{:?} + {:?} = {:?}", &p1, &p2, &p1 + &p2);
    println!("{:?} + {:?} = {:?}", p1, tup, p1 + tup);
    let total = &Point::new(10, 20) - &Point::new(100, 200);
    println!("{:#?}", total);
    let total = &Point::new(10, 20) - Point::new(100, 200);
    println!("{:#?}", total);
    let total = Point::new(10, 20) - &Point::new(100, 200);
    println!("{:#?}", total);
    let total = Point::new(10, 20) - Point::new(100, 200);
    println!("{:#?}", total);
}
