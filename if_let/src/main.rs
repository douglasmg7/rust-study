fn main() {
    let mut sauces = vec!["Mayonaise", "Ketchup", "Ranch"];

    if let Some(sauce) = sauces.pop() {
        println!("Sauce: {}", sauce)
    }

    if let Some(sauce) = sauces.pop() {
        println!("Sauce: {}", sauce)
    }

    if let Some(sauce) = sauces.pop() {
        println!("Sauce: {}", sauce)
    }

    if let Some(sauce) = sauces.pop() {
        println!("Sauce: {}", sauce)
    }
}
