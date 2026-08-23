pub mod products;

pub const FLOOR_SPACE: i32 = 10_000;
pub const MANAGER: &str = "Ivan Invetory";

pub fn talk_to_manager() {
    // Using relative path.
    println!("Hello {MANAGER}");
    // Using absolute path.
    println!("{}, how's your coffe?", crate::invetory::MANAGER);
}
