/*
    PRIMITIVE TYPES:
    bool: represents boolean, true or false
    char: represents a single unicode scalar value. char is 4 bytes in Rust, not just 1

    SLICES
*/

fn main() {
    let a = [1,2,3,4,5,6,7,8,9,10];
    let complete = &a[..]; // a slice containing all of the elements in 'a'.
    let middle = &a[1..4]; // a slice containing elements 1 through 3, not including 4 in 'a'.
    println!("The complete array: {}", complete[1]); // outputs 2
    println!("The middle of the array: {}", middle[0]); // also outputs 2, because [1..4] where element 0 in this slice = 2
}
