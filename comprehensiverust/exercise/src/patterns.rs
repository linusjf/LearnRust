pub fn main() {
    let s = "5sxq";

    for input in s.chars() {
        match input {
            'q' => println!("Quitting"),
            'a' | 's' | 'w' | 'd' => println!("Moving around"),
            '0'..='9' => println!("Number input"),
            _ => println!("Something else"),
        }
    }
    let nos = [100, 101];
    for n in nos {
        match divide_in_two(n) {
            Result::Ok(half) => println!("{n} divided in two is {half}"),
            Result::Err(msg) => println!("sorry, an error happened: {msg}"),
        }
    }
    destructs();
}

enum Result {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("cannot divide {n} into two equal parts"))
    }
}

struct Foo {
    x: (u32, u32),
    y: u32,
}

fn destructs() {
    let foos = [
        Foo { x: (1, 2), y: 3 },
        Foo { x: (2, 3), y: 2 },
        Foo { x: (4, 8), y: 6 },
    ];
    const Z: u32 = 2;
    for foo in foos {
        match foo {
            Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
            Foo { y: Z, x: i } => println!("y = 2, x = {i:?}"),
            Foo { y, .. } => println!("y = {y}, other fields were ignored"),
        }
    }
    let triples = [[0, -2, 3], [1, 2, 3], [4, 5, 6]];
    for triple in triples {
        println!("Tell me about {triple:?}");
        match triple {
            [0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
            [1, ..] => println!("First is 1 and the rest were ignored"),
            _ => println!("All elements were ignored"),
        }
    }
}
