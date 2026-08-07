fn select_two_first_cities<'a>(items: &'a [String]) -> &'a [String] {
    &items[0..2]
}

fn main() {
    let cities = vec![
        "Barcelona".to_string(),
        "Belo Horizonte".to_string(),
        "Brumadinho".to_string(),
    ];

    let two_cities = select_two_first_cities(&cities);
    println!("Cities: {:?}", two_cities);
}
