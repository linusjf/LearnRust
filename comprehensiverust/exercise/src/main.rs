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
    generics();
    conversions();
    forloops();
    printtranspose();
    variables();
    typeinferences();
    constants();
    banner();
    scopes();
    stackmemory();
    movesemantics();
    copyclone();
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

fn pick_one<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 {
        a
    } else {
        b
    }
}

fn generics() {
    println!("coin toss: {}", pick_one("heads", "tails"));
    println!("cash prize: {}", pick_one(500, 1000));
}

fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn conversions() {
    let x: i8 = 15;
    let y: i16 = 1000;
    println!("{x} * {y} = {}", multiply(x as i16, y));
    println!("{x} * {y} = {}", multiply(i16::from(x), y));
    println!("{x} * {y} = {}", multiply(x.into(), y));
}

fn forloops() {
    let array = [10, 20, 30];
    println!("array: {array:?}");
    print!("Iterating over array:");
    for n in array {
        print!(" {n}");
    }
    println!();

    print!("Iterating over range:");
    for i in 0..3 {
        print!(" {}", array[i]);
    }
    println!();
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j];
        }
    }
    return result;
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        println!("{row:?}");
    }
}

fn printtranspose() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}

fn variables() {
    let mut x: i32 = 10;
    println!("x: {x}");
    x = 20;
    println!("x: {x}");
}

fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn typeinferences() {
    let x = 10;
    let y = 20;
    takes_u32(x);
    takes_u32(y);
    takes_i8(y as i8);
    let mut v = Vec::new();
    v.push((10, false));
    v.push((20, true));
    println!("v: {v:?}");
    let vv = v.iter().collect::<std::collections::HashSet<_>>();
    println!("vv: {vv:?}");
}

const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    println!("digest: {digest:?}");
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        println!("b: {b:?}");
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

fn constants() {
    let digest = compute_digest("Hello");
    println!("Digest: {digest:?}");
}

static BANNER: &str = "Welcome to RustOS 3.14";

fn banner() {
    println!("{BANNER}");
}

fn scopes() {
    let a = 10;
    println!("before: {a}");

    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");

    let a = 1;
    let mut d = 10;
    let b = &a;
    let c = &mut d;
    let a = a + 1;
    *c = *c + 1;
    drop(c);
    println!("{a} {b} {d}");
}

fn stackmemory() {
    let mut s1 = String::from("Hello");
    s1.push(' ');
    s1.push_str("world");
    // DON'T DO THIS AT HOME! For educational purposes only.
    // String provides no guarantees about its layout, so this could lead to
    // undefined behavior.
    unsafe {
        let (capacity, ptr, len): (usize, usize, usize) = std::mem::transmute(s1);
        println!("ptr = {ptr:#x}, len = {len}, capacity = {capacity}");
    }
}

fn movesemantics() {
    let s1: String = String::from("Hello!");
    let s2: String = s1;
    println!("s2: {s2}");
    let s1: String = String::from("Hello!");
    let s2: String = s1.clone();
    println!("s2: {s2}");
    println!("s1: {s1}");

    fn say_hello(name: String) {
        println!("Hello, {name}!")
    }
    fn hello(name: &String) {
        println!("Hello, {name}!")
    }
    let name = String::from("Alice");
    say_hello(name);
    let name = &String::from("Alice");
    hello(name);
    hello(name);
}

#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

#[derive(Clone, Debug)]
struct Point2D(i32, i32, String);

fn copyclone() {
    let x = 42;
    let y = x;
    println!("x: {x}");
    println!("y: {y}");
    let p1 = Point(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
    let p1 = Point2D(3, 4, String::from("2D"));
    let p2 = p1.clone();
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}
