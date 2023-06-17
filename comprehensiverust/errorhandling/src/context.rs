use anyhow::{bail, Context, Result};
use std::io::Read;
use std::{fs, io};

fn read_username(path: &str) -> Result<String> {
    let mut username = String::with_capacity(100);
    fs::File::open(path)
        .with_context(|| format!("Failed to open {path}"))?
        .read_to_string(&mut username)
        .context("Failed to read")?;
    if username.is_empty() {
        bail!("Found no username in {path}");
    }
    Ok(username)
}

pub fn main() {
    //fs::write("user.dat", "").unwrap();
    match read_username("user.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err) => println!("Error: {err:?}"),
    }
    fs::write("user.dat", "").unwrap();
    match read_username("user.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err) => println!("Error: {err:?}"),
    }
    fs::write("user.dat", "Alice").unwrap();
    match read_username("user.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err) => println!("Error: {err:?}"),
    }
    fs::remove_file("user.dat").unwrap();
}
