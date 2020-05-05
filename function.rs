// Rust code uses snake case as the conventional style for function and variable names.
// In snake case, all letters are lowercase and underscores separate words.
fn main() {
    println!("First function!");
    first_function();
    another_function(4);
    println!("Five function: {}", five());
    println!("Plus one: {}", plus_one(3));
}

fn first_function() {
    println!("Second function!");
}

fn another_function(x: i32) {
    println!("Another function: {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}