fn no_car<'a, 'b>() -> Result<&'a str, &'b str> {
    // ? return the Err variant for the caller function.
    let car = Err("No car available")?;
    // The code below will not run.
    Ok(car)
}

fn some_car<'a, 'b>() -> Result<&'a str, &'b str> {
    // ? expand to value inside the variant Ok.
    let car = Ok("Monza")?;
    // Need be encapsulate again, to be of the same retuned type of the function.
    Ok(car)
}

fn main() {
    assert_eq!(no_car(), Err("No car available"));
    assert_eq!(some_car(), Ok("Monza"));
}
