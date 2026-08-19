#[derive(Debug)]
struct Food {
    name: String,
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        if self.has_mice_infestation {
            return None;
        }
        if self.reservations < 12 {
            Some(Food {
                name: "Sashimi".to_string(),
            })
        } else {
            Some(Food {
                name: "Strip Steak".to_string(),
            })
        }
    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            return Err("Sorry, we have a mice problem".to_string());
        } else if address.is_empty() {
            return Err("No delivery address specified".to_string());
        }
        Ok(Food {
            name: "Burger".to_string(),
        })
    }
}
fn main() {
    let res_1 = Restaurant {
        reservations: 11,
        has_mice_infestation: true,
    };
    println!("res1: {:?}", res_1.chef_special());
    println!("res1: {:?}", res_1.deliver_burger("123 Elm Street"));

    let res_2 = Restaurant {
        reservations: 15,
        has_mice_infestation: false,
    };
    println!("res1: {:?}", res_2.chef_special().unwrap().name);
    println!("res1: {:?}", res_2.deliver_burger(""));
    println!(
        "res1: {:?}",
        res_2.deliver_burger("Av Amazonas").unwrap().name
    );
}
