use std::error::Error;
use std::fs::{self, File};
use std::io::Read;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("Found no username in {0}")]
struct EmptyUsernameError(String);

fn read_username(path: &str) -> Result<String, Box<dyn Error>> {
    let mut username = String::with_capacity(100);
    File::open(path)?.read_to_string(&mut username)?;
    if username.is_empty() {
        return Err(EmptyUsernameError(String::from(path)).into());
    }
    Ok(username)
}

pub fn main() {
    //fs::write("confi.dat", "").unwrap();
    match read_username("confi.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err) => println!("Error: {err}"),
    }
    fs::write("confi.dat", "").unwrap();
    match read_username("confi.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err) => println!("Error: {err}"),
    }
    fs::write("confi.dat", "Alice").unwrap();
    match read_username("confi.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err) => println!("Error: {err}"),
    }
    fs::remove_file("confi.dat").unwrap();
}
