// Conditions and control flow

fn main() {
    let condition = (2_f32) <= 2.2;
    println!("2 is <= than 2.2: {}", condition);
    // logical operators:
    /*
    || - or
    && - and
    ! - not
    */
    /*
       operator precedence
       !
       &&
       ||
    */
    let food = "cookie";
    if (food == "cookie") {
        println!("I like cookies!");
    }
    if food == "cookie" {
        println!("I like cookies twice!");
    } else if food == "cake" {
        println!("I like cakes!");
    } else {
        println!("I don't like anything in the world!");
    }
}
