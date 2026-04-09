fn main() {
    let var1: i32 = 1_337;

    #[allow(unused_variables)]
    let var2: i16 = var1 as i16;

    let fl1: f64 = 3.456789;
    println!("fl1: {fl1:.3}");

    let with_milk: bool = true;
    let with_sugar: bool = true;
    let is_my_type_of_coffee: bool = with_milk && with_sugar;
    let is_acceptable_coffee: bool = with_milk || with_sugar;

    println!("is_my_type_of_coffee: {is_my_type_of_coffee}");
    println!("is_acceptable_coffee: {is_acceptable_coffee}");

    let ar: [i8; 4] = [1, 2, 3, 4];
    println!("ar: {ar:?}");

    let tup: (i32, f64, bool, [i8; 4]) = (4, 3.54, true, ar);
    println!("tup: {tup:?})");
}
