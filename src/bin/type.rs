use std::any::type_name_of_val;

fn main() {
    let numbers: Vec<i8> = vec![1, 2, 3];

    // Rust knows the outer type is Vec, and infers the inner type is i32
    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    println!("doubled: {:?}", type_name_of_val(&doubled));
}
