// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    call_me(3);
}

// i32 represents a 32-bit signed integer, while u32 represents a 32-bit unsigned integer. The difference is that i32 can hold both positive and negative numbers, while u32 can only hold non-negative numbers.
fn call_me(num: u32) { 
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
