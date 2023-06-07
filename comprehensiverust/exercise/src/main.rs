fn main() {
    hello();
    rawstrings();
    rawbytes();
    arrays();
    tuples();
    references();
    strings();
    print_fizzbuzz_to(100);
    methods();
}

fn hello() {
    println!("Edit me!");
    println!("Hello, 🌍!");
}

fn rawstrings() {
    println!(r#"<a href="link.html">link</a>"#);
    println!("<a href=\"link.html\">link</a>");
}

fn rawbytes() {
    println!("{:?}", b"abc");
    println!("{:?}", &[97, 98, 99]);
}

fn arrays() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {a:?}");
    println!("a: {a:#?}");
    println!("a: {:?}", a);
    println!("a: {:#?}", a);
}

fn tuples() {
    let t: (i8, bool) = (7, true);
    println!("1st index: {}", t.0);
    println!("2nd index: {}", t.1);
}

fn references() {
    let mut x: i32 = 10;
    let mut y: i32 = 20;
    let ref_x: &mut i32 = &mut x;
    *ref_x = 30;
    println!("ref_x count_ones: {:?}", ref_x.count_ones());
    println!("ref_x: {ref_x}");
    println!("x: {x}");
    let &mut ref_y: &mut i32 = &mut y;
    println!("y: {y}");
    println!("ref_y: {ref_y}");
    let ref_x: &i32;
    {
        let _x: i32 = 10;
        //  ref_x = &_x;
    }
    ref_x = &x;
    println!("ref_x: {ref_x}");
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");
    let s: &[i32] = &a[2..4];
    println!("s: {s:?}");
    a[0] = 90;
    println!("a: {a:?}");
}

fn strings() {
    let s1: &str = "World!";
    println!("s1: {s1}");
    let mut s2: String = String::from("Hello, ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");
    let s3: &str = &s2[7..];
    println!("s3: {s3}");
}

/// Determine whether the first argument is divisible by the second argument.
///
/// If the second argument is zero, the result is false.
fn is_divisible(n: u32, divisor: u32) -> bool {
    // corner case
    if divisor == 0 {
        return false;
    }
    // the last expression in a block is the return value
    n % divisor == 0
}

fn fizzbuzz(n: u32) -> String {
    let fizz = if is_divisible(n, 3) { "fizz" } else { "" };
    let buzz = if is_divisible(n, 5) { "buzz" } else { "" };
    if fizz.is_empty() && buzz.is_empty() {
        return format!("{n}");
    }
    format!("{fizz}{buzz}")
}

fn print_fizzbuzz_to(n: u32) {
    for i in 1..=n {
        print!(" {} ", fizzbuzz(i));
    }
    println!("")
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }

    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn new_square(width: u32) -> Rectangle {
        Rectangle {
            width,
            height: width,
        }
    }
}

fn methods() {
    let mut rect = Rectangle {
        width: 10,
        height: 5,
    };
    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());
    let mut rect = Rectangle::new(10, 5);
    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());
    let mut rect = Rectangle::new_square(10);
    println!("square area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());
}
