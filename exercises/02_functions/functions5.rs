// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}


// In Rust, the last expression in a function is implicitly returned. By adding a semicolon at the end of num * num;, you turn it into a statement, and the value is not returned. Removing the semicolon makes it an expression, and the result is returned.
fn square(num: i32) -> i32 {
    num * num
}
