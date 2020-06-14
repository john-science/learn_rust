use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Guess a number! (1-100)");

    loop {
        // get input from user (their guesses)
        let mut raw_guess = String::new();

        io::stdin()
            .read_line(&mut raw_guess)
            .expect("Failed to read line.");

        // try to parse the user-entered string as an integer. Handle bad entries.
        let guess: u64 = match raw_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("It doesn't look like you entered a valid number. Try again.");
                continue;
            }
        };

        // inform the user if they guessed too low or to high
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}

