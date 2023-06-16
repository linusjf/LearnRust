use std::fs::File;
use std::io::Read;
use std::panic;

fn main() {
    let file = File::open("diary.txt");
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            let _res = file.read_to_string(&mut contents);
            println!("Dear diary: {contents}");
        }
        Err(err) => {
            println!("The diary could not be opened: {err}");
        }
    }
    let result = panic::catch_unwind(|| {
        println!("hello!");
    });
    assert!(result.is_ok());

    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
    assert!(result.is_err());
    let v = vec![10, 20, 30];
    let result = panic::catch_unwind(|| {
        println!("v[100]: {}", v[100]);
    });
    assert!(result.is_err());
}
