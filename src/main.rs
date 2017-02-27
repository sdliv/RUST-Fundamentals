fn main() {
    let x: i32 = diverges();
}


fn diverges() -> ! {
    panic!("This function does not return");
}
