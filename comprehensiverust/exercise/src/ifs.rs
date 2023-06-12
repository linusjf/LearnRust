pub fn main() {
    let mut x = 10;
    if x % 2 == 0 {
        x = x / 2;
    } else {
        x = 3 * x + 1;
    }
    x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };
    println!("{:#?}", x);
    let arg = std::env::args().next();
    if let Some(value) = arg {
        println!("Program name: {value}");
    } else {
        println!("Missing name?");
    }
    println!("{:?}", second_word_to_upper("foo bar"));
    println!("{:?}", second_word_to_upper("bar"));
    let mut x = 10;
    while x != 1 {
        x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };
    }
    println!("Final x: {x}");
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();

    while let Some(x) = iter.next() {
        println!("x: {x}");
    }
    let v = vec![10, 20, 30];
    for x in v {
        println!("x: {x}");
    }
    for i in (0..10).step_by(2) {
        println!("i: {i}");
    }
    let mut v = vec![10, 20, 30];
    for x in v.iter_mut() {
        println!("x: {x}");
        *x = *x - 10;
        println!("x: {x}");
    }
    for x in v.iter_mut() {
        println!("x: {x}");
    }
    let mut x = 10;
    loop {
        x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };
        if x == 1 {
            break;
        }
    }
    println!("Final x: {x}");
    let binding = std::env::args().next();
    let x = binding.as_deref();
    match x {
        Some("cat") => println!("Will do cat things"),
        Some("ls") => println!("Will ls some files"),
        Some("mv") => println!("Let's move some files"),
        Some("rm") => println!("Uh, dangerous!"),
        None => println!("Hmm, no program name?"),
        _ => println!("Unknown program name: {:#?}", x),
    }
}

fn second_word_to_upper(s: &str) -> Option<String> {
    let mut it = s.split(' ');
    let (Some(_), Some(item)) = (it.next(), it.next()) else {
        return None;
    };
    Some(item.to_uppercase())
}
