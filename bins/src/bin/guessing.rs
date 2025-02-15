use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to the Guess the Number!");
    let random_number = rand::rng().random_range(1..=100);
    println!("The random number is {random_number}. 🤫");
    println!("Please guess a number > ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("user input should be a string");

    println!("You guessed: {}.", guess);

}