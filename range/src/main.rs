fn main() {
    // Go to 30.
    // let month_days = 1..31;
    let month_days: std::ops::Range<i32> = 1..31;
    // Go to 31.
    let month_days2 = 1..=31;
    println!("month_days: {month_days:?}");
    println!("month_days2: {month_days2:?}");

    for num in month_days2 {
        println!("{num}");
    }

    let letters = 'd'..'z';
    for letter in letters {
        println!("{letter}");
    }

    let states = ["MG", "RJ", "SP", "ES"];
    for state in states {
        println!("{state}");
    }
}
