fn main() {
    let a = [1, 2, 3, 4, 5];

    for item in a {
        println!("{item}");
    }

    // range reverse.
    for number in (1..4).rev() {
        println!("{number}");
    }
}
