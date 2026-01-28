use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello! Let's play the 'Guess the Number' game!");
    println!("I will pick a number between 1 and 100. Try to guess it.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("🎉 Congratulations! You guessed the number {}!", secret_number);
                break;
            }
        }
    }
}
