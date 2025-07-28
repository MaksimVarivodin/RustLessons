use std::io;

fn main() {
    // Setting up user input with Rust

    let mut input = String::new();
    // asking to enter a message
    println!("Please enter something: ");

    // reading line from console and add error message if something goes wrong
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // passing entered message to output
    println!("You entered: {}", input);
}
