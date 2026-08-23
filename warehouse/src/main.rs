mod invetory;
mod orders;

use invetory::products::productCategory::{Hammer, Ladder};
use invetory::products::{self, Item};
//use invetory::MANAGER;

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        orders::MANAGER,
        orders::MANAGER,
        invetory::FLOOR_SPACE
    );

    invetory::talk_to_manager();

    let favorite_category = Ladder;
    let other_category = Hammer;
    println!("Other category: {:?}", other_category);

    let tall_ladder = Item {
        name: String::from("Ladder-o-matic 200"),
        category: favorite_category,
        quantity: 100,
    };
    println!("{:#?}", tall_ladder);
}
