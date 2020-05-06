fn main() {
    // Derive the `fmt::Debug` implementation for `Structure`.
    // `Structure` is a structure which contains a single `i32`.
    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    // Using default debug print.
    println!("Structure: {:?}", Structure(3));
    println!("Deep: {:?}", Deep(Structure(7)));
    println!("Structure: {:#?}", Structure(3));

    // Using default debug print with pretty.
    println!("Structure: {:#?}", Structure(3));
    println!("Deep: {:#?}", Deep(Structure(7)));
}