#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: i32,
    hard_drive_capacity: i32,
}

impl Computer {
    fn new(cpu: String, memory: i32, hard_drive_capacity: i32) -> Self {
        Self {
            cpu,
            memory,
            hard_drive_capacity,
        }
    }

    fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        self
    }

    fn upgrade_memory(&mut self, new_memory: i32) -> &mut Self {
        self.memory = new_memory;
        self
    }

    fn upgrade_hard_drive_capacity(&mut self, new_hard_drive_capacity: i32) -> &mut Self {
        self.hard_drive_capacity = new_hard_drive_capacity;
        self
    }
}

fn main() {
    let mut computer = Computer::new(String::from("Intel I7"), 32, 256);
    computer
        .upgrade_cpu(String::from("Intel I5"))
        .upgrade_memory(64)
        .upgrade_hard_drive_capacity(512);

    println!("{computer:#?}");
}
