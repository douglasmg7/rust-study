fn main() {
    let mut cars = Vec::<&str>::new();
    cars.push("Chevette");
    cars.push("Kadet");
    cars.insert(1, "Monza");

    let last_car = cars.pop().unwrap_or("");

    println!("{cars:?}");
    println!("Last car: {last_car:?}");

    let first_car = cars.remove(0);
    println!("First car: {first_car:?}");

    let car1 = "Chevette".to_string();
    let car2 = "Monza".to_string();
    let car3 = "Kadet".to_string();
    let mut cars = vec![car1, car2, car3];
    cars.push("Cobalt".to_string());

    // Only can get by ref.
    let ref_car2 = &cars[2];
    println!("ref_car2: {ref_car2}");

    // All cars.
    println!("cars: {cars:?}");

    // Slice.
    let slice_cars = &cars[0..2];
    println!("slice_cars: {slice_cars:?}");

    // Using get().
    let second_car = cars.get(1).unwrap();
    println!("Second car: {second_car}")
}
