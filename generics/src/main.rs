use std::fmt::Debug;

fn make_tuple<T>(first: T, second: i32) -> (T, i32) {
    (first, second)
}

fn make_tuple_one<T>(first: T, second: T) -> (T, T) {
    (first, second)
}

fn make_tuple_two<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

#[allow(dead_code)]
#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

// Specific implemantation for a concrate type (String)
#[allow(dead_code)]
impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string();
    }
}

// Specific implemantation for a concrate type (slice)
#[allow(dead_code)]
impl TreasureChest<[&str; 3]> {
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}

// Geral implemantation, no concrate type specified
impl<T> TreasureChest<T> {
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

fn main() {
    // The first type is a gneric, can be any type.
    println!("tuple: {:?}", make_tuple("um", 3));

    // The two parameters need be of the same type.
    println!("tuple: {:?}", make_tuple_one("um", "dois"));

    // Which parameter can be of different types or the same type.
    println!("tuple: {:?}", make_tuple_two("um", 3));
    println!("tuple: {:?}", make_tuple_two("um", "3"));

    let mut golden_chest = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: String::from("   Golen  "),
    };
    golden_chest.clean_treasure();

    let special_chest = TreasureChest {
        captain: String::from("Bearbread"),
        treasure: ["Golden", "Silver", "Water"],
    };

    println!("golden_chest: {:?}", golden_chest);
    println!("Capital case name: {}", golden_chest.capital_captain());
    println!("special_chest: {:?}", special_chest);
    println!("Treasure quantity: {}", special_chest.amount_of_treasure());
    println!("Capital case name: {}", special_chest.capital_captain());
}
