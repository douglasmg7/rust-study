fn main() {
    let mut trip = start_trip();

    visit_philadelphia(&mut trip);
    trip.push_str(" and ");
    visit_new_york(&mut trip);
    trip.push_str(" and ");
    visit_boston(&mut trip);
    trip.push_str(".");
    show_initinerary(&trip);
    // dbg!(t1);
}

fn start_trip() -> String {
    String::from("The plan is...")
}

fn visit_philadelphia(text: &mut String) {
    text.push_str("Philadelphia");
}

fn visit_new_york(text: &mut String) {
    text.push_str("New York");
}

fn visit_boston(text: &mut String) {
    text.push_str("Boston");
}

fn show_initinerary(text: &String) {
    println!("{text}");
}
