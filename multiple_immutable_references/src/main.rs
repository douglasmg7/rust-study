fn main() {
    let car = String::from("Blue");
    let ref1 = &car;
    let ref2 = &car;

    println!("Car color: {car}");
    println!("ref1 color: {ref1}");
    println!("ref2 color: {ref2}");
    println!("Car color: {car}");
}