struct Location {
    name: String,
    treasure: u32,
}

struct Map<'a> {
    locations: &'a [Location],
}

impl<'a> Map<'a> {
    fn explore<F>(&self, mut action: F)
    where
        F: FnMut(&Location),
    {
        for loct in self.locations.iter() {
            action(loct)
        }
    }
}

fn main() {
    let locations = [
        Location {
            name: "Enchanted Forest".to_string(),
            treasure: 3,
        },
        Location {
            name: "Mystic Mountain".to_string(),
            treasure: 8,
        },
        Location {
            name: "Banga 440".to_string(),
            treasure: 4,
        },
        Location {
            name: "Forth Beach".to_string(),
            treasure: 9,
        },
    ];

    let map = Map {
        locations: &locations,
    };

    let mut total_treasure = 0;
    let mut location_names: Vec<String> = Vec::new();
    map.explore(|location| {
        total_treasure += location.treasure;
        location_names.push(location.name.clone());
    });

    println!("Total treasure: {}", total_treasure);
    println!("Location names: {:?}", location_names);
}
