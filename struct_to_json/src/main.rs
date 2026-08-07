use serde::Serialize;

// 1. Derive the Serialize trait on your struct
#[derive(Serialize)]
struct User {
    username: String,
    age: u32,
    is_active: bool,
}

fn main() {
    // 2. Create an instance of your struct
    let user = User {
        username: String::from("alice_dev"),
        age: 30,
        is_active: true,
    };

    // 3. Convert the struct to a compact JSON string
    match serde_json::to_string(&user) {
        Ok(json_string) => println!("Compact: {}", json_string),
        Err(e) => eprintln!("Failed to serialize: {}", e),
    }

    // 4. Alternatively, convert to a pretty-printed JSON string
    let pretty_json = serde_json::to_string_pretty(&user).unwrap();
    println!("Pretty:\n{}", pretty_json);
}
