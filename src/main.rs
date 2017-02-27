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
    let x = true; // type inference
    let y: bool = false; // without type inference

    let char_1 = 'x';
    let two_hearts = '💕';

    println!("Here are both chars: {} and the two hearts: {}", char_1, two_hearts);
}
