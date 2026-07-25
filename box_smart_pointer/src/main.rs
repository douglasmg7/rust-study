fn main() {
    let my_box = Box::new(111);
    println!("Value of my_box: {}", my_box);
    println!("Value of my_box: {}", *my_box);
}
