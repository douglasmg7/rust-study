use fake::Dummy;

#[derive(Debug, Dummy)]
pub enum productCategory {
    Ladder,
    Hammer,
}

#[derive(Debug, Dummy)]
pub struct Item {
    pub name: String,
    pub category: productCategory,
    pub quantity: u32,
}

impl Item {
    pub fn new(name: String, category: productCategory, quantity: u32) -> Self {
        super::talk_to_manager();
        Self {
            name,
            category,
            quantity,
        }
    }
}
