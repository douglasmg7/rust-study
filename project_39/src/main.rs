const TOUCHDOWN_POINTS: i32 = 6;

#[allow(unused_variables)]
fn main() {
    let season: &str = "Pizza";
    let mut points_scored: i32 = 28;
    println!("points_scored: {points_scored}");
    points_scored = 35;
    let event_time: &str = "06:00";
    let event_time: i32 = 6;

    println!(
        "season: {season}\npoints_scored: {}\nevent_time:{1}, TOUCHDOWN_POINTS: {2}",
        points_scored, event_time, TOUCHDOWN_POINTS
    );

    #[allow(unused_variables)]
    let favoreite_beverage: &str = "Watter";
}
