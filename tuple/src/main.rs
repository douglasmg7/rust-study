fn main() {
    // Type annotations.
    let tup: (i32, f64, u8) = (500, 6.1, 1);
    // Access a tuple element.
    println!("The value of x is {}", tup.0);

    // Infering type.
    let tup = (500, 6.1, 1);

    // Destructing.
    let (_, y, z) = tup;
    println!("The value of y is {}", y);
    println!("The value of z is {}", z);

    // Unit tuple, retunr implicit by function that is supposed to remove nothing.
    let unit: () = ();
    println!("The unit tuple: {:?}", unit);
}