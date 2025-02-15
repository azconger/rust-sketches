use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the Guess the Number!");
    let random_number = rand::rng().random_range(1..=100);
    println!("The random number is {random_number}. 🤫");
    loop {
        println!("Please guess a number between 1 and 100 > ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("user input should be a string");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error: {e:}");
                continue
            }
        };

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small! ☹️"),
            Ordering::Equal => {
                println!("You win! 🤩");
                break
            },
            Ordering::Greater => println!("Too big! ☹️"),
        }
    }
}
