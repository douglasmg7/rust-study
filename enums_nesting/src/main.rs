#[derive(Debug)]
enum Meat {
    Chicken,
    Steak,
}

#[derive(Debug)]
enum RestaurantItem {
    Burrito(Meat),
    Bowl(Meat),
    VeganPlate,
}

fn main() {
    let lunch = RestaurantItem::Burrito(Meat::Steak);
    let dinner = RestaurantItem::Bowl(Meat::Chicken);
    let light = RestaurantItem::VeganPlate;
    println!("Lunch: {lunch:?}");
    println!("Light: {light:?}");
}
