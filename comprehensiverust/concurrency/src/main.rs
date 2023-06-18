use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Count in thread: {i}!");
            thread::sleep(Duration::from_millis(5));
            panic!();
        }
    });

    let s = String::from("Hello");

    thread::spawn(move || {
        println!("Length: {}", s.len());
        thread::sleep(Duration::from_millis(5));
    });
    let s = String::from("Hello");

    thread::scope(|scope| {
        scope.spawn(|| {
            println!("Length: {}", s.len());
        });
    });
    for i in 1..5 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(5));
    }
    let res = handle.join();
    println!("{:#?}", res);
}
