fn main() {
    // &str: This is a string slice, which is a reference to a sequence of UTF-8 encoded bytes stored elsewhere in memory. It is immutable and always points to a valid UTF-8 sequence. String slices are commonly used for borrowing string data without taking ownership. They are often seen as function parameters where the function needs to operate on string data without consuming it.
    let mut fixed_string: &str = "This is a string";
    println!("fixed_string: {fixed_string}");

    fixed_string = "This string";
    println!("fixed_string: {fixed_string}");

    fixed_string = "Another string";
    println!("fixed_string: {fixed_string}");

    // String: This is a growable, heap-allocated string. It owns the string data it contains and can be modified. String is more flexible and allows dynamic modification of the string contents. It is commonly used when you need to build or manipulate strings dynamically.
    let mut flexible_string: String = String::from("ula ula");
    flexible_string.push('s');
    flexible_string.push_str(" ULA");
    println!("flexible_string: {flexible_string}");

    // In summary, &str is a borrowed reference to a string slice, while String is an owned, heap-allocated string. Use &str when you need to borrow string data without taking ownership, and use String when you need to own and manipulate string data dynamically.

    // Do the same thing, so which you choose is a matter of style and readability.
    let s1 = "Hello ".to_string();
    let s2 = String::from("word");

    // you can conveniently use the + operator or the format! macro to concatenate String values.
    let s1 = s1 + &s2;
    let s1 = format!("{s1}!");
    println!("{s1}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    s1.push_str("---");
    s1.push('!');
    println!("s2 is {s2}");
    println!("s1 is {s1}");
}