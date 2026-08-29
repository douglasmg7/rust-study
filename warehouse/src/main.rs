mod invetory;
mod orders;

use fake::{Fake, Faker};

use invetory::products::productCategory::{Hammer, Ladder};
use invetory::products::{self, Item};
use invetory::MANAGER as INVENTORY_MANAGER;
use orders::MANAGER as ORDERS_MANAGER;

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        INVENTORY_MANAGER,
        ORDERS_MANAGER,
        invetory::FLOOR_SPACE
    );

    //invetory::talk_to_manager();
    //
    //let favorite_category = Ladder;
    //let other_category = Hammer;
    //println!("Other category: {:?}", other_category);
    //
    //let tall_ladder = Item::new(String::from("Ladder-o-matic 200"), favorite_category, 100);
    //
    //println!("{:#?}", tall_ladder);

    let fake_item: Item = Faker.fake();
    println!("{:#?}", fake_item);

    let random_category: products::productCategory = Faker.fake();
    println!("{:#?}", random_category);
}
