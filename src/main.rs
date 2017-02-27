/*
    PRIMITIVE TYPES:
    bool: represents boolean, true or false
    char: represents a single unicode scalar value. char is 4 bytes in Rust, not just 1

    NUMERIC TYPES:
    Categories of numeric types:
    1. signed and unsigned
    2. fixed and variable
    3. floating-point and integer
*/

fn main() {
    // Arrays: they are immutable by default, can be made mutable, can hold type, and limits be set

    let a = [1,2,3]; // a:[i32; 3]
    let mut m = [1,2,3]; // m:[i32; 3]
    let c = [0; 20];  // short hand initializing of an array of 20 elements, all set to 0
    let names = ["Sean", "Nicole", "Joanna", "Adoniah"]; // names: [&str; 3]

    println!("The length of array c, is: {}", c.len());
    println!("The second name in the 'names' array is: {}", names[1]);
}
