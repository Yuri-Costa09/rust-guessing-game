use std::io::{stdin};

fn main() {
    println!("Play the guessing game!");

    println!("Guess a number: ");

    let mut guess = String::new();

    stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed! the number was: {guess}")
        
}
