mod frominto;
mod iterators;

use std::fmt::Display;
trait Pet {
    fn name(&self) -> String;
}

struct Dog {
    name: String,
}

struct Cat;

impl Pet for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Pet for Cat {
    fn name(&self) -> String {
        String::from("The cat") // No name, cats won't respond to it anyway.
    }
}

fn greet<P: Pet>(pet: &P) {
    println!("Who's a cutie? {} is!", pet.name());
}

fn main() {
    let fido = Dog {
        name: "Fido".into(),
    };
    greet(&fido);

    let captain_floof = Cat;
    greet(&captain_floof);
    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Cat),
        Box::new(Dog {
            name: String::from("Fido"),
        }),
    ];
    for pet in pets {
        println!("Hello, {}!", pet.name());
    }
    println!(
        "{} {}",
        std::mem::size_of::<Dog>(),
        std::mem::size_of::<Cat>()
    );
    println!(
        "{} {}",
        std::mem::size_of::<&Dog>(),
        std::mem::size_of::<&Cat>()
    );
    println!("{}", std::mem::size_of::<&dyn Pet>());
    println!("{}", std::mem::size_of::<Box<dyn Pet>>());
    let p1 = Player::default();
    let p2 = p1.clone();
    println!(
        "Is {:?}\nequal to {:?}?\nThe answer is {}!",
        &p1,
        &p2,
        if p1 == p2 { "yes" } else { "no" }
    );
    let a = Centimeter(10);
    let b = Centimeter(20);
    println!("{a:?} equals {b:?}: {}", a.equal(&b));
    println!("{a:?} not_equals {b:?}: {}", a.not_equal(&b));
    println!("{a:?} equal {b:?}: {}", a.equals(&b));
    println!("{a:?} not_equal {b:?}: {}", a.not_equal(&b));
    println!("{a:?} equal {b:?}: {}", a.equals(&b));
    println!("{a:?} not_equal {b:?}: {}", a.not_equals(&b));
    let foo = String::from("foo");
    let pair = duplicate(foo);
    println!("{pair:?}");
    let many = add_42_millions(42_i8);
    println!("{many}");
    let many_more = add_42_millions(10_000_000);
    println!("{many_more}");
    let x = get_x("foo");
    println!("{x}");
    iterators::main();
    frominto::main();
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}

trait Equals {
    fn equal(&self, other: &Self) -> bool;
    fn not_equal(&self, other: &Self) -> bool {
        !self.equal(other)
    }
}

#[derive(Debug)]
struct Centimeter(i16);

impl Equals for Centimeter {
    fn equal(&self, other: &Centimeter) -> bool {
        self.0 == other.0
    }
}

impl Equal for Centimeter {
    fn equals(&self, other: &Centimeter) -> bool {
        self.0 == other.0
    }
}

trait Equal {
    fn equals(&self, other: &Self) -> bool;
}

trait NotEqual: Equal {
    fn not_equal(&self, other: &Self) -> bool {
        !self.equals(other)
    }
}

trait NotEquals {
    fn not_equals(&self, other: &Self) -> bool;
}

impl<T> NotEquals for T
where
    T: Equals,
{
    fn not_equals(&self, other: &Self) -> bool {
        !self.equal(other)
    }
}

fn duplicate<T: Clone>(a: T) -> (T, T)
where
    T: Clone,
{
    (a.clone(), a.clone())
}

// Syntactic sugar for:
//   fn add_42_millions<T: Into<i32>>(x: T) -> i32 {
fn add_42_millions(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}

fn get_x(name: impl Display) -> impl Display {
    format!("Hello, {name}!")
}
