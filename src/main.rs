fn main() {

    /*
        VARIABLE BINDING
    */
    let x = 5; // example of Variable Binding using 'type inference.' x: i32
    
    /*
        PATTERNS
    */
    let (y, z) = (1, 2); // Left hand side of a let statement is a 'pattern', not a variable name; uses 'type inference'
    
    /*
        TYPE ANNOTATIONS
    */
    let a: i32 = 5; // a is a binding with type 'i32' and the value '5'.  This is a 32 bit signed integer. This is 'type annotation'
    
    /*
        'i' is for signed integers
        'u' is for unsigned integers
        Possible integer sizes: 8, 16, 32, and 64 bit
    */

    /*
        The following code will not compile
        Because bindings in Rust are immutable by default.
        
        let b = 5;
        b = 10;

        You will receive the following error:
        error: re-assignment of immutable variable 'x'
            x = 10;
            ^~~~~~~
    */
   
    /*
        MUTABLE
    */
    // For the above to be mutable, you must use the 'mut' keyword
    let mut b = 5; // mut x: i32
    x = 10;

    /*
        INITIALIZING BINDINGS:
        Variable bindings must have a value when they are initialized before you're allowed to use them.
    */

    // let x: i32; will not work.
    /* You will receive the following error:
        warning: unused variable: 'x', #[warn(unused_variables)] on by default.
        src/main.rs:2   let x: i32;
                            ^
    */
}