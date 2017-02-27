fn main() {
    // Scope and shadowing
    let x: i32 = 17;
    {
        let y: i32 = 3;
        println!("The value of x is {} and the value of y is {}", x, y);
    }
    println!("The value of x is {} and the value of y is {}", x, y); // This will not work due to y being out of scope

}