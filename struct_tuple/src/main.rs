#[derive(Debug)]
struct Color(u32, u32, u32);

fn main() {
    let red = Color(1, 2, 3);
    print_color(&red);
}

fn print_color(color: &Color) {
    println!("Red: {}", color.0);
    println!("Red: {}", color.1);
    println!("Red: {}", color.2);
}
