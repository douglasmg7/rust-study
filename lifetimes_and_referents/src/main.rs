fn select_two_first_cities<'a>(items: &'a [String]) -> &'a [String] {
    &items[0..2]
}

fn choose_favorite<'a>(first: &'a str, second: &str) -> &'a str {
    println!("{second}");
    first
}

fn choose_longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

fn main() {
    let cities = vec![
        "Barcelona".to_string(),
        "Belo Horizonte".to_string(),
        "Brumadinho".to_string(),
    ];

    let two_cities = select_two_first_cities(&cities);
    println!("Cities: {:?}", two_cities);

    println!("Returned vale: {:?}", choose_favorite("aaa", "bbb"));

    let orlando = "Orlando".to_string();
    let north_caroline = "North Caroline".to_string();

    let longest_one = choose_longest(&orlando, &north_caroline);
    println!("Longest value : {:?}", longest_one);
}
