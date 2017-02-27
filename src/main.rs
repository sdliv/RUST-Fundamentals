fn main() {
    // Shadowing
    let x: i32 = 8;
    {
        println!("{}", x); // Prints "8".
        let x = 12;
        println!("{}", x); // Prints "12".
    }
    println!("{}", x); // Prints "12".
    let x = 42;
    println!("{}", x); // Prints "42".

    let mut y: i32 = 1;
    y = 7; // y is now the value of 7
    let y = y; // 'y' is now immutable and is bound to '7';

    let z = 4;
    let z = "I can also be bound to text!"; // 'y' is now of a different type, due to the 'let' rebinding of 'z'

}