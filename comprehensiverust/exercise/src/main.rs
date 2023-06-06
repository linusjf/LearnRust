fn main() {
    hello();
    rawstrings();
    rawbytes();
    arrays();
    tuples();
    references();
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
