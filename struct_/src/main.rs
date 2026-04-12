struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)] // Create debug trait.
struct Coffee {
    name: String,
    price: f64,
    is_hot: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);
    println!("{}", user1.username);
    println!("{}", user1.active);
    println!("{}", user1.sign_in_count);

    let email = user1.email;
    println!("email: {email}");
    // println!("{}", user1.email);     Not valid anymore.
    println!("{}", user1.username);

    let _mocha = Coffee {
        name: String::from("Mocha"),
        price: 4.59,
        is_hot: true,
    };
    println!("_mocha debug trait: {:?}", _mocha);
    println!("_mocha debug trait pretty: {:#?}", _mocha);

    let mocha = make_coffee(String::from("Mocha"), 34.5, true);
    println!(
        "Coffee name: {}, price {}, is_hot {}",
        mocha.name, mocha.price, mocha.is_hot
    );

    // Using mocha coffee to set some fields.
    let caramel_mocha = Coffee {
        name: String::from("Caramel mocha"),
        ..mocha
    };
    println!(
        "Coffee name: {}, price {}, is_hot {}",
        caramel_mocha.name, caramel_mocha.price, caramel_mocha.is_hot
    );
}

// Create coffe.
fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot,
    }
}