fn main() {
    // Array have defined size.
    let names: [&str; 3] = ["Júlio", "Marcia", "Marcos"];
    for name in names {
        println!("{name}");
    }

    // Type and size array are infered.
    let cars = ["Monza", "Chevette", "Cobalt"];
    println!();
    for car in cars {
        println!("{car}")
    }

    let mut numbers: Vec<i32> = Vec::new();
    println!();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    for number in numbers {
        println!("{number}");
    }

    let mut names = Vec::<&str>::new();
    println!();
    names.push("Flávia");
    names.push("Patrícia");
    names.push("Caroline");
    for name in names {
        println!("{name}");
    }

    let cars = vec!["Passat", "Voyage", "Gol"];

    println!("\n{cars:?}");
}
