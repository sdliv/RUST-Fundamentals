/* 
    Conditionals
*/

fn main() {
    let x = 5;

    if x == 5 {
        println!("x = 5");
    } else if x == 6 {
        println!("x = 6");
    } else {
        println!("x != 5");
    }

    // Time for some tricks

    let z = 7;
    let y = if z == 5 {
        10
    } else {
        15
    }; // y: i32

    // Here is a better way to write the previous conditional:

    let a = 7;
    let b = if a == 7 {10} else {15}; // b: i32
}

