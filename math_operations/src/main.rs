fn main() {
    // let i1: i32 = 5 / 2;
    // let f1: f32 = 5. / 2.;
    println!("Integer dividsion: {}", 5 / 2);
    println!("Integer modulos: {}", 5 % 2);
    println!("Float dividsion: {}", 5. / 2.);

    // Augmented assignment operators.
    let mut i1: i32 = 1;
    i1 += 1;
    println!("i1: {}", i1);

    let b1: bool = true;
    let b2: bool = false;

    println!("{}", b1 && b2);
    println!("{}", b1 || b2);
    println!("{}", !b1);
    println!("{}", i1 > 3);
    println!("{}", i1.is_negative());
}
