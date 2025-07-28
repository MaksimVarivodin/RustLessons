/*
Common Programming Concepts in Rust
YouTube: https://www.youtube.com/watch?v=2V0JaMVjzws&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=4
06.02.25
*/

fn main() {
    // 1.

    //+++++++++++++++++++++++++++++++++++++++++++
    // Immutability
    //+++++++++++++++++++++++++++++++++++++++++++

    /*
    let x: i32 = 5;
    println!("{}", x);
    x = 6; cannot assign twice to immutable variable
    */

    // 2.

    //+++++++++++++++++++++++++++++++++++++++++++
    // Mutability
    //+++++++++++++++++++++++++++++++++++++++++++

    let mut x: i32 = 5;
    println!("x is {}", x);
    x = 6;
    println!("x is {}", x);

    // 3.

    //+++++++++++++++++++++++++++++++++++++++++++
    // Constants
    //+++++++++++++++++++++++++++++++++++++++++++

    // Consts can't be mutated. Can be equal to const expressions
    const SUBSCRIBER_COUNT: u32 = 100_000;
    println!("SUBSCRIBER_COUNT :{}", SUBSCRIBER_COUNT);

    // 4.

    //+++++++++++++++++++++++++++++++++++++++++++
    // Shadowing
    //+++++++++++++++++++++++++++++++++++++++++++

    let x: i32 = 5;
    println!("x is {}", x);
    let x = "string";
    println!("x is {}", x);

    // 5.

    //+++++++++++++++++++++++++++++++++++++++++++
    // Scalar datatypes
    //+++++++++++++++++++++++++++++++++++++++++++

    // Integers
    // Floating-point numbers
    // Boolean
    // Characters

    // Integers
    let a: i32 = 98_222; // Decimal
    let b: i32 = 0xff; // Hexadecimal
    let c: i32 = 0o77; // Octal
    let d: i32 = 0b1111_0000; // Binary
    let e: u8 = b'A'; // Byte (u8 only) ASCII character

    // * integer overflow
    //let f: u8 = 256;
    /*
                error: literal out of range for `u8`
      --> src\main.rs:76:25
       |
    76 |             let f: u8 = 256;
       |                         ^^^
       |
       = note: the literal `256` does not fit into the type `u8` whose range is `0..=255`
       = note: `#[deny(overflowing_literals)]` on by default

               */

    // Floating point numbers

    // * default type for floating point numbers is f64
    let f = 2.0;
    let g: f32 = 3.0;
    
    // Numeric Operations
    
    // addition 
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication 
    let product = 4* 30;
    // division
    let quotient = 56.7 /32.2;
    // remainder
    let remainder = 43 % 5;
    
    // Booleans
    
    let t = true;
    
    let f: bool = false;
    
    // Character 
    
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
    
}
