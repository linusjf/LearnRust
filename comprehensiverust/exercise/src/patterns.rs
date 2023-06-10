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
