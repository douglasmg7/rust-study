// Unit struct.
struct Empty;

impl Empty {
    fn print_message(&self) {
        println!("Just a message");
    }
}

fn main() {
    let empty = Empty;
    empty.print_message();
}
