
use std::io;

pub fn new() -> u32 {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    return guess;
}