fn main() {
    // Difference between mut and immutable variable

    // immutable variable
    let x: u128 = 4;
    println!("x = {}", x);

    {
        // doesn't change out-scope
        let x: u128 = 50;
        println!("inner after\nx = {}", x);
    }
    let x: u128 = 5;
    println!("after\nx = {}", x);

    // mutable variable
    let mut y: u64 = 5;
    println!("y = {}", y);

    y = 6;
    println!("after\ny = {}", y);

    // constants
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {}", MAX_POINTS);
}
