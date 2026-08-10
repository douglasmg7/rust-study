#[derive(Debug, Clone, Copy)]
enum MyOption<T> {
    Some(T),
    None,
}

impl<T> MyOption<T> {
    fn unwrap(self) -> T {
        match self {
            Self::Some(val) => val,
            Self::None => panic!("No value."),
        }
    }
    fn unwrap_or(self, default_value: T) -> T {
        match self {
            Self::Some(val) => val,
            Self::None => default_value,
        }
    }
}

fn main() {
    let some_value = MyOption::Some("dog");
    let none_value = MyOption::<&str>::None;

    println!("none_value: {}", none_value.unwrap_or("Default"));

    println!("some_value: {}", some_value.unwrap());
    println!("none_value: {}", none_value.unwrap());
}
