use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

fn game() {
    println!("Guessing the number >>> ");
    {
        println!("hello world")
    }
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input your guess: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please Enter a number!".red().bold());
                continue;
            }
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small!".red()),
            Ordering::Greater => println!("{}", "Too Big!".red()),
            Ordering::Equal => {
                println!("{}", "Exactly!".green());
                break;
            }
        };
    }
}

fn main() {
    game();
}
