fn main() {
    let mut a = 3;
    println!("a: {}", a);
    let r = &mut a;
    println!("r: {}", *r);
    *r = 1;
    println!("r: {}", *r);
    println!("a: {}", a);
    println!("r: {}", *r);
}