fn main() {
    let is_concert: bool = true;
    let is_event = is_concert;

    println!("is_event: {is_event}");
    println!("is_concert: {is_concert}");

    let mut sushi = String::from("Salmon");
    // let dinner = sushi;

    eat_meal(&mut sushi);

    println!("sushi: {sushi}");
    // println!("dinner: {dinner}");
}

fn eat_meal(meal: &mut String) {
    println!("Meal: {meal}");
    meal.clear();
    println!("Meal: {meal}");
}
