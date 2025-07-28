fn main() {
    // Data types

    // Integer types

    // signed integer
    let integer1: i8 = 1;
    let integer2: i16 = 2;
    let integer3: i32 = 3;
    let integer4: i64 = 4;
    let integer5: i128 = 5;
    let integer6: isize = 6;
    println!(
        "i8 = {},\n\
         i16 = {},\n\
         i32 = {},\n\
         i64 = {},\n\
         i128 = {},\n\
         isize = {}",
        integer1, integer2, integer3, integer4, integer5, integer6
    );

    // unsigned integer
    let uinteger1: u8 = 1;
    let uinteger2: u16 = 2;
    let uinteger3: u32 = 3;
    let uinteger4: u64 = 4;
    let uinteger5: u128 = 5;
    let uinteger6: usize = 6;

    println!(
        "u8 = {},\n\
         u16 = {},\n\
         u32 = {},\n\
         u64 = {},\n\
         u128 = {},\n\
         usize = {}",
        uinteger1, uinteger2, uinteger3, uinteger4, uinteger5, uinteger6
    );
    //__________________________________________________________

    // Floating number types

    // floating point type with 32 bit precision
    let float1: f32 = 1.0;

    // floating point type with 64 bit precision
    let float2: f64 = 2.0;

    println!("f32 = {},\nf64 = {}", float1, float2);

    //__________________________________________________________

    // Non-numeric types

    // Boolean type
    let boolean1: bool = true;

    // Character type
    let character1: char = 'a';

    // String type
    let string1: String = "Hello, world!".to_string();

    println!(
        "bool = {},\nchar = {},\nstring = {}",
        boolean1, character1, string1
    );
    //__________________________________________________________

    // Container types

    // Tuple

    let tup: (i32, bool, char) = (500, true, 'z');
    // access to elements:
    println!("tup.0 = {},\ntup.1 = {},\ntup.2 = {}", tup.0, tup.1, tup.2);
    // or
    let (x, y, z) = tup;
    println!("x = {},\ny =\n{},\nz = {}", x, y, z);

    // Array

    // hardcode initialization

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!(
        "arr[0] = {},\narr[1] = {},\narr[2] = {},\narr[3] = {},\narr[4] = {}",
        arr[0], arr[1], arr[2], arr[3], arr[4]
    );

    // init with a value

    let arr2: [i32; 5] = [0; 5];
    println!(
        "arr2[0] = {},\narr2[1] = {},\narr2[2] = {},\narr2[3] = {},\narr2[4] = {}",
        arr2[0], arr2[1], arr2[2], arr2[3], arr2[4]
    );
}
