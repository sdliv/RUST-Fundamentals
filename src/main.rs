/*
    COMMENTS:
    Rust has line comments and 'doc' comments.  Doc comments have three '/' and support markdwon
*/

fn main() {
    let x = 5; // this is a line comment
    println!("Adding one to {} is {}", x, add_one(x));
}


// Example of doc comment below:

/// Add one to the given number
/// 
/// # Examples
///
/// ```
///
/// let five = 5;
///
/// assert_eq!(6, add_one(5));
/// # fn add_one(x: i32) -> i32 {   
/// #     x + 1
/// # }
/// ```

fn add_one(x: i32) -> i32 {
    x + 1
}