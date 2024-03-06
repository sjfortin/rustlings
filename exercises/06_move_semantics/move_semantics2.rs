// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(&vec0); // ampersand means reference

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> { // the &Vec is a reference
    let mut new_vec = vec.clone();

    new_vec.push(88);

    new_vec
}
