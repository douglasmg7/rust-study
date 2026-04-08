use std::mem;

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

    // Char - 4bytes.
    let c1: char = 'S';
    println!("{c1}");
    println!("{}", c1.is_alphabetic());
    println!("{}", mem::size_of::<char>());

    // Arrray.
    let a1: [i32; 3] = [1, 2, 3];
    let a2 = [4, 5, 6];
    println!("a1: {}", a1[0]);
    // Using debug trait, array do not implement de display trait.
    println!("a1: {:?}", a1);
    println!("a1: {a1:?}");
    // Pretty print debug.
    println!("a1: {a1:#?}");
    println!("a2: {}", a2[2]);
    println!("a1 length: {}", a1.len());

    // Debug macro:
    dbg!(a1.len());
}
