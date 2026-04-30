enum Milk {
    LowFat(i32),
    Whole,
    NonDairy { kind: String },
}

fn main() {
    let beverage = Milk::LowFat(5);
    // let beverage = Milk::Whole;

    let Milk::LowFat(percent) = beverage else {
        println!("No low fat milk.");
        return;
    };
    println!("Low fat {percent}% milk available.");
}
