pub const FLOOR_SPACE: i32 = 10_000;
pub const MANAGER: &str = "Ivan Invetory";

#[derive(Debug)]
pub enum productCategory {
    Ladder,
    Hammer,
}

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub category: productCategory,
    pub quantity: u32,
}

pub fn talk_to_manager() {
    println!("Hey {MANAGER}, how's your coffe?");
}
