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
}

fn second_word_to_upper(s: &str) -> Option<String> {
    let mut it = s.split(' ');
    let (Some(_), Some(item)) = (it.next(), it.next()) else {
        return None;
    };
    Some(item.to_uppercase())
}
