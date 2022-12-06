use std::io::{self, Read};
pub mod grid;
pub mod intcode_computer;

pub fn get_input() -> io::Result<String> {
    let mut s = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut s)?;
    Ok(s)
}
