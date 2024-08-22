use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    //let _first = String::from("Rust"); instance of String, use when you dont know the value of the string ahead of the time
    let correct = rand::thread_rng().gen_range(1..=10);
    //println!("(correct: {})", correct);
    println!("Hey,guess a number 1-10:");

    loop {
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

        match guess.cmp(&correct) {
            Ordering::Greater => println!("You guessed too high."),
            Ordering::Less => println!("You guessed too low!"),
            Ordering::Equal => {
                println!("You guessed the correct number.");
                break;
            }
        };
    }
}

// let _last = "Programming"; Reference to the string slice
//println!("Hello, {first} {}!", last.to_lowercase());
//let data = [1, 2, 3, 4, 5];
//println!("{data:?}") //format in debug format
