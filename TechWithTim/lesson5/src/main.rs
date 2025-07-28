use std::io;

fn main() {
    // Arithmetic operations

    // Binary ranges example:

    //let x: u8 = 256; // 0 - 255
    /*
         --> src\main.rs:3:17
      |
    3 |     let x: u8 = 256; // 0 - 255
      |                 ^^^
      |
      = note: the literal `256` does not fit into the type `u8` whose range is `0..=255`
      = note: `#[deny(overflowing_literals)]` on by default
        */

    let x: u8 = 255; // 0 - 255
    let y: i8 = 3; // (-128) - 127

    // Type casting / conversion

    //let z = x + y;
    /*
        error[E0308]: mismatched types
      --> src\main.rs:21:17
       |
    21 |     let z = x + y;
       |                 ^ expected `u8`, found `i8`

        */
    let z = (x as u16) + (y as u16);
    println!("z = {}", z);

    // Input casting

    println!("Enter number:");
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");

    let converted_number: u32 = input_string.trim().parse().unwrap();
    println!("Input number: {}", input_string);
    println!("Converted number: {}", converted_number);
}
