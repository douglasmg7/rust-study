fn main() {
    // Arrays.
    let a: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    println!("Fisrt element of array a: {}", a[0]);
    println!("All elements of array a: {:?}", a);

    // Init array values.
    let a: [i32; 10] = [6; 10];
    println!("All elements of modified array a: {:?}", a);
}