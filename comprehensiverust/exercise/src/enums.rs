use std::mem;

fn generate_random_number() -> u32 {
    std::process::id()
}

#[derive(Debug)]
enum CoinFlip {
    Heads,
    Tails,
}

fn flip_coin() -> CoinFlip {
    let random_number = generate_random_number();
    if random_number % 2 == 0 {
        return CoinFlip::Heads;
    } else {
        return CoinFlip::Tails;
    }
}

enum WebEvent {
    PageLoad,                 // Variant without payload
    KeyPress(char),           // Tuple struct variant
    Click { x: i64, y: i64 }, // Full struct variant
}

#[rustfmt::skip]
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad       => println!("page loaded"),
        WebEvent::KeyPress(c)    => println!("pressed '{c}'"),
        WebEvent::Click { x, y } => println!("clicked at x={x}, y={y}"),
    }
}

enum Foo {
    A(&'static str),
    B(i32),
    C(i32),
}

pub fn main() {
    println!("You got: {:?}", flip_coin());
    let load = WebEvent::PageLoad;
    let press = WebEvent::KeyPress('x');
    let click = WebEvent::Click { x: 20, y: 80 };

    println!("{:#?}", mem::discriminant(&load));
    println!("{:#?}", mem::discriminant(&press));
    println!("{:#?}", mem::discriminant(&click));
    inspect(load);
    inspect(press);
    inspect(click);
    assert_eq!(
        mem::discriminant(&Foo::A("bar")),
        mem::discriminant(&Foo::A("baz"))
    );
    assert_eq!(mem::discriminant(&Foo::B(1)), mem::discriminant(&Foo::B(2)));
    assert_ne!(mem::discriminant(&Foo::B(3)), mem::discriminant(&Foo::C(3)));
}
