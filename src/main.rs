use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let mut how_many = String::new();
    println!("How many times do you want to guess?");
    io::stdin()
        .read_line(&mut how_many)
        .expect("Failed to read line");

    let num_guesses: u8 = how_many
        .trim()
        .parse()
        .expect("Please enter a valid number.");

    let mut correct = Vec::new();

    for _ in 0..num_guesses {
        correct.push(rand::thread_rng().gen_range(1..=10));
    }

    //println!("{correct:?}");

    let mut guesses_made = 0;

    println!("Please enter your guess.");

    while guesses_made < num_guesses {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        match guess.cmp(&correct[guesses_made as usize]) {
            Ordering::Greater => println!("You guessed too high."),
            Ordering::Less => println!("You guessed too low!"),
            Ordering::Equal => {
                println!("You guessed the correct number.");
                guesses_made += 1;
                if guesses_made < num_guesses {
                    println!("Please enter your next guess.");
                }
            }
        };
    }
    println!("You have guessed all the numbers correctly. The correct answers were:");

    for item in correct {
        println!("{item}");
    }
}

// let _last = "Programming"; Reference to the string slice
//println!("Hello, {first} {}!", last.to_lowercase());
//let data = [1, 2, 3, 4, 5];
//println!("{data:?}") //format in debug format
