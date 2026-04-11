fn main() {
    let mut burguer = String::from("Burguer");

    add_fries(&mut burguer);
    println!("Burguer: {burguer}");
}

// meal: String
// mut meal: String
// meal: &String
// meal: &mut String

fn add_fries(meal: &mut String) {
    meal.push_str(" and Fries");
}