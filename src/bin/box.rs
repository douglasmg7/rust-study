fn main() {
    let b = Box::new(55_u64);
    println!("b: {}", *b);
    println!("b: {}", b);
}
