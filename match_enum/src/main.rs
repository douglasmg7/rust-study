enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
}

enum LaundryCycle {
    Cold,
    Hot { temperature: i32 },
    Delicate(String),
}

impl LaundryCycle {
    fn wash_laundry(&self) {
        match self {
            LaundryCycle::Cold => {
                println!("Cold cycle")
            }
            LaundryCycle::Hot { temperature } => {
                println!("Running the laundry with temperature of {temperature}")
            }
            LaundryCycle::Delicate(cloth) => {
                println!("Running the laundry with cloth {cloth}")
            }
        }
    }
}

fn main() {
    let linux = OperatingSystem::Linux;
    println!("OS year realese: {}", years_since_releas(&linux));

    let cycle_type = LaundryCycle::Delicate(String::from("Linho"));
    wash_laundry(&cycle_type);
    wash_laundry(&LaundryCycle::Hot { temperature: 32 });
    wash_laundry(&LaundryCycle::Cold);

    LaundryCycle::Cold.wash_laundry();
    let hot_cycle = LaundryCycle::Hot { temperature: 45 };
    hot_cycle.wash_laundry();
}

fn years_since_releas(os: &OperatingSystem) -> i32 {
    match os {
        OperatingSystem::Windows => 39,
        OperatingSystem::MacOS => 23,
        OperatingSystem::Linux => {
            println!("Linux OS");
            34
        }
    }
}

fn wash_laundry(cycle: &LaundryCycle) {
    match cycle {
        LaundryCycle::Cold => {
            println!("Cold cycle")
        }
        LaundryCycle::Hot { temperature } => {
            println!("Running the laundry with temperature of {temperature}")
        }
        LaundryCycle::Delicate(cloth) => {
            println!("Running the laundry with cloth {cloth}")
        }
    };
}
