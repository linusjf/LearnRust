fn main() {
    println!("Edit me!");
    println!("Hello, 🌍!");
    println!(r#"<a href="link.html">link</a>"#);
    println!("<a href=\"link.html\">link</a>");
    println!("{:?}", b"abc");
    println!("{:?}", &[97, 98, 99]);
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {a:?}");
    println!("a: {a:#?}");
    println!("a: {:?}", a);
    println!("a: {:#?}", a);
    let t: (i8, bool) = (7, true);
    println!("1st index: {}", t.0);
    println!("2nd index: {}", t.1);
}
