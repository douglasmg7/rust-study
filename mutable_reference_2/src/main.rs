fn main() {
    let mut car = String::from("Blue");

    let mref1 = &mut car;
    mref1.push_str(" and Green");

    // Can not borrow here. Mutable reference used after this point.
    // let ref2 = &car;

    println!("mref1: {mref1}");
    println!("car: {car}");

    let ref3 = &car;
    println!("ref3: {ref3}");
}
