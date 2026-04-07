use std::io;

fn main() {
    let mut distance_from_home = String::new();
    io::stdin()
        .read_line(&mut distance_from_home)
        .expect("Fail to read line");
    // Variable shadowing, now is a new variable with different type using the same name.
    // The previous was discarted.
    let distance_from_home: i32 = distance_from_home.trim().parse().expect("Not a number");

    println!("Distance from home: {distance_from_home}");
}
