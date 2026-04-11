fn main() {
    println!("Hello, world!");
    let mut ar = [1, 2, 3, 4, 5];
    let ar_slice = &mut ar[2..4];

    ar_slice[0] = 89;
    println!("ar_slice: {ar_slice:?}");
    println!("ar: {ar:?}");
}
