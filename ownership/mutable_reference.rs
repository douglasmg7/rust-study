fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    // But mutable references have one big restriction: you can have only one mutable reference to a particular piece of data in a particular scope.
    // let r1 = &mut s;
    // let r2 = &mut s;

    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}