#[derive(Debug)]
struct Flight {
    origen: String,
    destination: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new(origen: String, destination: String, price: f64, passengers: u32) -> Self {
        Flight {
            origen,
            destination,
            price,
            passengers,
        }
    }

    fn change_destination(&mut self, destination: String) {
        self.destination = destination;
    }

    fn increase_price(&mut self) {
        self.price *= 1.2;
    }

    fn itinerary(&self) {
        println!("({}->{})", self.origen, self.destination);
    }
}

fn main() {
    let mut flight = Flight::new(String::from("Floripa"), String::from("Canadá"), 10000.0, 4);
    flight.change_destination(String::from("Brasil"));
    flight.increase_price();
    flight.itinerary();
    println!("{:#?}", flight);

    let fl2 = Flight {
        origen: String::from("Panamá"),
        destination: String::from("Cuiabá"),
        ..flight
    };
    println!("{:#?}", fl2);
}
