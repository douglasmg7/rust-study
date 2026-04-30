enum Milk {
    Whole,
    LowFat(i32),
    NoDairy { kind: String },
}

fn main() {
    let my_beverage = Milk::Whole;
    let my_low_fat = Milk::LowFat(3);
    let my_orange_juice = Milk::NoDairy {
        kind: String::from("Orange Juice"),
    };
    let my_kale_juice = Milk::NoDairy {
        kind: String::from("Kale Juice"),
    };

    if let Milk::Whole = my_beverage {
        println!("My whole milk");
    }

    if let Milk::LowFat(fat_percentage) = my_low_fat {
        println!("My Low Fat {fat_percentage}%");
    }

    if let Milk::LowFat(2) = my_low_fat {
        println!("My Low Fat 2%");
    } else {
        println!("You have other beverage");
    }

    if let Milk::NoDairy { kind } = my_orange_juice {
        println!("My no dairy beverage is {kind}");
    }

    if let Milk::NoDairy { kind } = my_kale_juice {
        if kind == "Kale Juice" {
            println!("My Kale Juice");
        }
    }
}
