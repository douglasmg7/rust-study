const FLOOR_SPACE: i32 = 10_000;
pub const MANAGER: &str = "Ivan Invetory";

#[derive(Debug)]
enum productCategory {
    Ladder,
    Hammer,
}

#[derive(Debug)]
struct Item {
    name: String,
    category: productCategory,
    quantity: u32,
}

fn talk_to_manager() {
    println!("Hey {MANAGER}, how's your coffe?");
}
