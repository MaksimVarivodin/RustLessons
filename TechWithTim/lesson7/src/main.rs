// Functions
fn main() {
    test_function();
}

// no parameters
fn test_function() {
    println!("Test function called");
    print_sum(1, 2);
    println!("Return value: {}", return_sum(1, 2));
    println!("Return value: {}", return_sum2(1, 2));
}

// some parameters example
fn print_sum(a: i32, b: i32) {
    println!("a + b = {}", a + b);
}

// return statement
fn return_sum(a: i32, b: i32) -> i32 {
    a + b
}
fn return_sum2(a: i32, b: i32) -> i32 {
    return a + b;
}
