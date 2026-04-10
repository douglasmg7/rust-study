fn main() {
    let a = [1, 2, 3, 4, 5];

    for item in a {
        println!("{item}");
    }
    println!("\n");

    // range reverse.
    for number in (10..14).rev() {
        println!("{number}");
    }
    println!("\n");

    // range reverse.
    for number in (10..=14).rev() {
        println!("{number}");
    }
    println!("\n");
}
