// Constant can be defined global.
// Constants using the const keyword instead of the let keyword, and the type of the value must be annotated.
// Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.
// The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.
const PI: f64 = 3.14;
fn main() {
    const ASDF: i32 = 3 * 30 + 4;
    println!("PI: {PI}");
    println!("ASDF: {ASDF}");
}
