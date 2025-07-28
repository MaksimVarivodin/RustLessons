fn main() {
    // Memory management,Heap & Stack
    let x = 2;
    let y = x;

    example();
    string_creation();
}

fn example() {
    let a = "true";
    let b = false;
}
fn string_creation() {
    let string = String::from("hello");
    println!("string is {}", string);
}

/*
        stack at line 12, during execution
bottom
            name        value
            x           2
            y           2
            a           "true"
            b           false
top

        stack at line 6, during execution
bottom
            name        value
            x           2
            y           2
top
*/
