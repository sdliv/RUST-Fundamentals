fn main() {
    print_number(4);
    add_numbers(16, 17);
    let x = add_one(7);
    println!("x is: {}", x);
}

fn print_number(x: i32) {
    println!("The number you entered is: {}", x);
}

fn add_numbers(x: i32, y: i32) {
    // let z = x + y;
    println!("The sum is {}", x + y)
}

fn add_one(x: i32) -> i32 {
    x + 1 // functions return the last evaluated statement.  If it is to be returned, it does not end with a semi colon.
    // return x+1; works as well, because 'return' is a keyword, but it is considered poor style.
}

/*
    DIVERGING functions
*/

fn diverges() -> ! {
    panic!("This function never returns");
}
