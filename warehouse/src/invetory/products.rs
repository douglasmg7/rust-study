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
