fn excute_thrice<F>(procedure: F)
where
    F: Fn(),
{
    procedure();
    procedure();
    procedure();
}

fn main() {
    let some_string = "Earth";
    excute_thrice(|| println!("I am here in the {}.", some_string));
}
