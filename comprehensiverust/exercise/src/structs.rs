#![allow(unused_variables, dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
impl Person {
    fn new(name: String, age: u8) -> Person {
        Self { name, age }
    }
}

impl Default for Person {
    fn default() -> Person {
        Person {
            name: "Bot".to_string(),
            age: 0,
        }
    }
}

fn create_default() -> Person {
    let tmp = Person {
        ..Default::default()
    };
    println!("{:#?}", tmp);
    let tmp = Person {
        name: "Sam".to_string(),
        ..Default::default()
    };
    println!("{:#?}", tmp);
    tmp
}

struct Point(i32, i32);

struct PoundsOfForce(f64);
struct Newtons(f64);

fn compute_thruster_force() -> PoundsOfForce {
    // todo!("Ask a rocket scientist at NASA")
    return PoundsOfForce(64.0);
}

fn set_thruster_force(_force: PoundsOfForce) {
    // ...
}

pub fn main() {
    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    println!("{} is {} years old", peter.name, peter.age);

    peter.age = 28;
    println!("{} is {} years old", peter.name, peter.age);

    let jackie = Person {
        name: String::from("Jackie"),
        ..peter
    };
    println!("{} is {} years old", jackie.name, jackie.age);
    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);
    let force = compute_thruster_force();
    set_thruster_force(force);
    let peter = Person::new(String::from("Peter"), 27);
    println!("{peter:?}");
    create_default();
}
