use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    // generate secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("secret {}", secret_number);
    // welcome message
    println!("Guess a number!");
    println!("Please input your guess.");

    // receive user input
    loop {
        let mut guess = String::new();
        // receive user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // convert to i32 guess string
        let guess: u32 = match guess.trim().parse() {
            Ok(num) =>num,
            Err(_) =>continue,
        }; 

        // compare
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
