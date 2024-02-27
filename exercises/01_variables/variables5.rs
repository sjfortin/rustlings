// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3; // this is an example of shadowing https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing
    println!("Number plus two is : {}", number + 2);
}
