use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number game!");
    println!("Do you want challenge mode? Enter 'c' if yes, or anything else if no.");

    let mut challengeInput = String::new();

    io::stdin()
    .read_line(&mut challengeInput)
    .expect("Failed to read line");

    let challengeMode = challengeInput.trim().eq_ignore_ascii_case("c"); // True if challengeInput = 'c' case insensitive

    if challengeMode {
        println!("Challenge mode activated! You have 5 attempts to guess the number.");
    } else {
        println!("Normal mode activated! You can guess as many times as you want.");
    }

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guessesRemaining = if challengeMode { 5 } else { 0 };
    
    loop {
        if challengeMode {
            if guessesRemaining == 0 {
                println!("You lose! The secret number was {secret_number}");
                break;
            } else {
                println!("You have {guessesRemaining} attempts remaining.");
            }
        }
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid guess. Guess must be a number!");
                continue;
            },
        };
    
        println!("You guessed: {guess}");
        if challengeMode {
            guessesRemaining -= 1;
        }
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! The secret number was {guess}");
                break;   
            }
        }
    }
}
