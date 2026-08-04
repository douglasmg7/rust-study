enum Cheesesteak<T> {
    Plain,
    Topping(T),
}

fn main() {
    let mushroom = Cheesesteak::Topping("mushroom");

    let topping: String = "bacon".to_string();
    // Reference to String, not &str
    let bacon: Cheesesteak<&String> = Cheesesteak::Topping(&topping);

    // Must define type because rustc could not infer the type of T
    let mut plain: Cheesesteak<&str> = Cheesesteak::Plain;
    plain = Cheesesteak::Topping("Onion");

    println!("Hello, world!");
}
