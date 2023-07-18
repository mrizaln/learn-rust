use std::io::Read;
use std::{fs, io};

#[allow(dead_code)]
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

fn read_username_try(path: &str) -> Result<String, io::Error> {
    let mut username_file = fs::File::open(path)?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

pub fn main() {
    // fs::write("config.dat", "alice").unwrap();
    // let username = read_username("config.dat");
    let username = read_username_try("config.dat");
    println!("username or error: {username:?}");
}
