use std::collections::HashMap;

fn main() {
    options();
    strings();
    vectors();
    hashmaps();
    boxed();
}

fn options() {
    let numbers = vec![10, 20, 30];
    let first: Option<&i8> = numbers.first();
    println!("first: {first:?}");
    let idx: Result<usize, usize> = numbers.binary_search(&10);
    println!("idx: {idx:?}");
    let idx: Result<usize, usize> = numbers.binary_search(&15);
    println!("idx: {idx:?}");
}

fn strings() {
    let mut s1 = String::new();
    s1.push_str("Hello");
    println!("s1: len = {}, capacity = {}", s1.len(), s1.capacity());

    let mut s2 = String::with_capacity(s1.len() + 1);
    s2.push_str(&s1);
    s2.push('!');
    println!("s2: len = {}, capacity = {}", s2.len(), s2.capacity());

    let s3 = String::from("🇨🇭");
    println!(
        "s3: len = {}, number of chars = {}",
        s3.len(),
        s3.chars().count()
    );
    let s3 = &*s1;
    println!("{:#?}", s3);
    println!("{:#?}", s1.chars().nth(3).unwrap());
    //    println!("{:#?}", s1.chars().nth(6).unwrap());
}

fn vectors() {
    let mut v1 = Vec::new();
    v1.push(42);
    println!("v1: len = {}, capacity = {}", v1.len(), v1.capacity());

    let mut v2 = Vec::with_capacity(v1.len() + 1);
    v2.extend(v1.iter());
    v2.push(9999);
    println!("v2: len = {}, capacity = {}", v2.len(), v2.capacity());

    // Canonical macro to initialize a vector with elements.
    let mut v3 = vec![0, 0, 1, 2, 3, 4];

    // Retain only the even elements.
    v3.retain(|x| x % 2 == 0);
    println!("{v3:?}");

    // Remove consecutive duplicates.
    v3.dedup();
    println!("{v3:?}");
    for i in 0..10 {
        print!("{:#?} ", v3.get(i));
    }
    println!();
    for e in &mut v3 {
        *e += 50;
    }
    for i in 0..10 {
        print!("{:#?} ", v3.get(i));
    }
    println!();
}

fn hashmaps() {
    let mut page_counts = HashMap::new();
    page_counts.insert("Adventures of Huckleberry Finn".to_string(), 207);
    page_counts.insert("Grimms' Fairy Tales".to_string(), 751);
    page_counts.insert("Pride and Prejudice".to_string(), 303);

    if !page_counts.contains_key("Les Misérables") {
        println!(
            "We know about {} books, but not Les Misérables.",
            page_counts.len()
        );
    }

    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        match page_counts.get(book) {
            Some(count) => println!("{book}: {count} pages"),
            None => println!("{book} is unknown."),
        }
    }

    // Use the .entry() method to insert a value if nothing is found.
    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        let page_count: &mut i32 = page_counts.entry(book.to_string()).or_insert(0);
        *page_count += 1;
    }

    println!("{page_counts:#?}");

    let pc1 = page_counts
        .get("Harry Potter and the Sorcerer's Stone ")
        .unwrap_or(&336);
    println!("{pc1:#?}");
    let pc2 = page_counts
        .entry("The Hunger Games".to_string())
        .or_insert(374);
    println!("{pc2:#?}");

    let page_counts = HashMap::from([
        ("Harry Potter and the Sorcerer's Stone".to_string(), 336),
        ("The Hunger Games".to_string(), 374),
    ]);
    println!("{page_counts:#?}");
    let mut page_counts = HashMap::from([
        ("Harry Potter and the Sorcerer's Stone", 336),
        ("The Hunger Games", 374),
    ]);
    println!("{page_counts:#?}");
    let pc1 = page_counts
        .get("Harry Potter and the Sorcerer's Stone")
        .unwrap_or(&336);
    println!("{pc1:#?}");
    let pc2 = page_counts.entry("The Hunger Games").or_insert(374);
    println!("{pc2:#?}");
}

fn boxed() {
    let five = Box::new(5);
    println!("five: {}", five);
    println!("five: {}", *five);
}
