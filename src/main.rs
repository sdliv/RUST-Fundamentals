/*
    FUNCTION POINTERS
*/

fn main() {
    let f: fn(i32) -> i32 = plus_one; // Without type inference
    let g = plus_one; // With type inference

    let seven = f(6);
    let eight = g(7);
    println!("using the function f(6) = {}\nusing the function g(7) = {}", seven, eight);

}

fn plus_one(i: i32) -> i32 {
    i + 1
}
