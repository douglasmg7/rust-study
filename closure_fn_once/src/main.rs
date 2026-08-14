use std::io::stdin;

struct Vault {
    treasure: String,
    password: String,
}

impl Vault {
    fn unlock<F>(self, procedure: F) -> Option<String>
    where
        F: FnOnce() -> String,
    {
        let user_password = procedure();
        if self.password == user_password {
            return Some(self.treasure);
        }
        None
    }
}

fn main() {
    let chest = Vault {
        treasure: "Gold".to_string(),
        password: "someone".to_string(),
    };

    let mut pass = "".to_string();
    println!("password to unlock the treasure: ");
    stdin().read_line(&mut pass).expect("Some password");
    let pass = pass.trim().to_string();

    let treasure = chest.unlock(|| pass).unwrap_or("Locked".to_string());

    println!("treasure: {}", treasure);
}
