extern crate rand;

mod get_guess;

use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let mut guesses = 0;

    loop {
        if guesses == 5 {
            println!("Out of guesses");
            break;
        }

        println!("Please input your guess.");

        let random = rand::thread_rng().gen_range(1, 11);
        let guess = get_guess::new();

        match guess.cmp(&random) {
            Ordering::Less => println!("Too small. The number was {}", random),
            Ordering::Greater => println!("Too big. The number was {}", random),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }

        guesses += 1;
    }
}