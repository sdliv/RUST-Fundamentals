/*
    PRIMITIVE TYPES:
    bool: represents boolean, true or false
    char: represents a single unicode scalar value. char is 4 bytes in Rust, not just 1

    TUPLES: an ordered list with fixed size.
*/

fn main() {
    let x = (1, "hello"); // with inference
    let y: (i32, &str) = (2, "hello"); //  without type inference

    /*
        You can assign one tuple into another
        if they have the same contained types and arity
        Tuples have the same airty when they have the same
        length.
    */

    let mut z = (1, 2); // z: (i32, i32)
    let t = (3, 4);

    z = t;

    let (r, s) = t;
    println!("r is {}", r);

    /*
        Tuple Indexing
    */

    let tuple = (1, 2, 3);

    let i = tuple.0;
    let j = tuple.1;
    let k = tuple.2;

    println!("The value of i: {}\nThe value of j: {}\nThe value of k: {}", i, j, k);

    /*
        And somehow, FUNCTIONS CAN HAVE TYPES!
    */

    fn foo(x: i32) -> i32 { x }

    let a: fn(i32) -> i32 = foo;
    // in this case 'a' is a 'function pointer' to a function that takes an i32 and returns an i32.
}
