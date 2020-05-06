/*
Here, we used fmt::Display because the std library provides implementations for these types.
fmt::Display: Uses the {} marker.
To print text for custom types, more steps are required.
*/
fn main() {
    println!("This month has {} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}", object="the lazy dog", subject="The quick brown fox", verb="jumps over");
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    println!("{number:>width$}", number=1, width=20);
    println!("{:>width$}", 2, width=20);
}
