// No associated values.
#[derive(Debug)]
enum CardSuit {
    Hearts,
    Dimonds,
    Spades,
    Clubs,
}

// // With associated values.
// #[derive(Debug)]
// struct Credentials {
// username: String,
// password: String,
// }

// With associated values.
#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String, u32),
    DebitCard(String),
    // Struct variant.
    PayPal { username: String, password: String },
}

fn main() {
    let mut cards = CardSuit::Hearts;
    println!("cards: {:?}", cards);
    cards = CardSuit::Clubs;
    println!("cards: {:?}", cards);

    let payment = PaymentMethodType::CreditCard(String::from("3434239482"), 54u32);
    let payment2 = PaymentMethodType::DebitCard(String::from("3434239482"));
    let payment3 = PaymentMethodType::PayPal {
        username: String::from("asdf@asdf.com"),
        password: String::from("euio398"),
    };

    println!("payment: {:?}", payment);
    println!("payment2: {:?}", payment2);
    println!("payment3: {:?}", payment3);
}
