fn main() {
    let i: i16 = 3;
    // Can define data type at end too.
    let i2 = 45i16;
    let i3 = 1_23_456_3_45u64;
    println!("i:\t{i}\ni2:\t{i2}\ni3\t{i3}\n");

    // In a 64 bits pc it is alias for i64.
    let var1: usize = 100;
    println!("var1: {var1}");

    // String literals, defined at compile time, unmtable.
    println!("Hello World!");
    let s1: &str = "Ula lá";
    println!("{s1}");

    // Raw string.
    let s2: &str = r"c:\files\internals";
    println!("{s2}");
}
