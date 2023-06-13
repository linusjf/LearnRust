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
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}
