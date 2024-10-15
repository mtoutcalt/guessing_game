use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Guessing Game!");

    loop {
        play_game();
        println!("Do you want to play again? (y/n)");

        let mut play_again = String::new();
        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");

        if play_again.trim().to_lowercase() != "y" {
            println!("Thanks for playing dude!");
            break;
        }
    }
}

fn play_game() {
    println!("I'm thinking of a number between 1 and 100.");

    let secret_number = rand::thread_rng().gen_range(1..=100); //inclusive

    // Initialize the guess counter
    let mut guess_count = 0;

    loop {
        println!("Please input your guess.");

        guess_count += 1;

        let mut guess = String::new();

        // Read the user's input and handle any errors
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the input String to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // Compare the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("🎉 You win!");
                println!("You guessed the number in {} attempts.", guess_count);
                break;
            }
        }
    }
}