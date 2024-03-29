use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::{self};
use std::panic;
mod context;
mod convert;
mod deriveerror;
mod dynamicerror;

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
    let username = read_username("config.dat");
    println!("username or error: {username:?}");
    fs::write("config.dat", "").unwrap();
    let username = read_username("config.dat");
    println!("username or error: {username:?}");
    fs::write("config.dat", "alice").unwrap();
    let username = read_username("config.dat");
    println!("username or error: {username:?}");
    convert::main();
    deriveerror::main();
    dynamicerror::main();
    context::main();
    fs::remove_file("config.dat").unwrap();
}

fn read_username(path: &str) -> Result<String, io::Error> {
    let username_file_result = fs::File::open(path);
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(err) => return Err(err),
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(err) => Err(err),
    }
}
