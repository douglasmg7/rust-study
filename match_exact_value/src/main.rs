enum Milk {
    LowFat(i32),
    Whole,
}

impl Milk {
    fn drink(&self) {
        match self {
            Milk::LowFat(2) => {
                println!("Very low fat milk!");
            }
            Milk::LowFat(fat_percentage) => {
                println!("{fat_percentage}% fat Milk");
            }
            Milk::Whole => {
                println!("Whole Milk");
            }
        }
    }
}

fn main() {
    Milk::LowFat(2).drink();
    Milk::LowFat(5).drink();
    Milk::Whole.drink();
}
