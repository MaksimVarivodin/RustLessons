/*

Programming a Guessing Game in Rust!
YouTube lesson: https://www.youtube.com/watch?v=H0xBSbnQYds&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=2
06.02.25

*/

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {
    println!("Guess the number!");

    let mut rng = rand::rng();

    let secret_number = rng.random_range(1..10);

    loop {
        println!("{}", "Please input your guess:".yellow());

        let mut string_guess = String::new();

        io::stdin()
            .read_line(&mut string_guess)
            .expect("Failed to read line");



        let numeric_guess: u32 = match string_guess.trim().parse(){
            // handling non-numeric values
            Ok(num) => {
                println!("{}", format!("You guessed: {}", string_guess).yellow());
                num
            },
            Err(_) => {
                println!("{}","Please input a number!".truecolor(255, 128, 0));
                continue;
            },
        };


        match numeric_guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".blue()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
