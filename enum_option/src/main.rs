fn main() {
    let mut some_value: Option<&str>;

    some_value = Option::None;
    //some_value = Option::Some("Ha!");

    match some_value {
        Some(val) => println!("Some value: {}", val),
        None => println!("Nothing here"),
    }

    //println!("Some value unwrapped: {}", some_value.unwrap());
    println!(
        "Some value unwrapped: {}",
        some_value.expect("No value found.")
    );

    let ar = [81, 92, 103];
    let index = 13;
    match ar.get(index) {
        Some(val) => println!("Value at position {}: {}", index, val),
        None => println!("No value at position {}", index),
    }
}
