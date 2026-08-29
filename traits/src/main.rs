use std::collections::HashMap;

trait Accommodation {
    // A default method defined.
    fn get_description(&self) -> String {
        String::from("A wonderfull place to stay.")
    }
    // Need a explict implementation.
    fn book(&mut self, name: &str, nights: u32);
}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }

    fn summarize(&self) -> String {
        // Will use default method from the trait Accommodation.
        format!("{}: {}", self.name, self.get_description())
    }
}

impl Accommodation for Hotel {
    //fn get_description(&self) -> String {
    //    format!("{} is the pinnacle of luxury", self.name)
    //}

    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

#[derive(Debug)]
struct AirBnB {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnB {
    fn new(host: &str) -> Self {
        AirBnB {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for AirBnB {
    fn get_description(&self) -> String {
        format!("{} is the best AirBnB host!", self.host)
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}

fn book_for_one_night(entity: &mut impl Accommodation, guest: &str) {
    entity.book(guest, 1);
}

fn main() {
    let mut hotel = Hotel::new("The Luxe");
    println!("{}", hotel.summarize());
    hotel.book("Lúcia", 3);
    book_for_one_night(&mut hotel, "Marcos");
    println!("{:#?}", hotel);

    let mut airbnb = AirBnB::new("Peter");
    println!("{}", airbnb.get_description());
    airbnb.book("Marcos", 4);
    book_for_one_night(&mut airbnb, "Júlia");
    println!("{:#?}", airbnb);
}
