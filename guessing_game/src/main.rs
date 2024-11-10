use std::cmp::Ordering;
use std::io;

use rand::Rng;

/// A simple guessing game. The program will generate a random number between 1 and 100 (inclusive).
/// The user is then prompted to guess the number. The program will print out the user's guess.
fn main() {
    println!("Guess The Number!");

    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}