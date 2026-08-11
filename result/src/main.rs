fn main() {
    let ok: Result<i32, &str> = Result::Ok(569);
    let some_err: Result<i32, &str> = Result::Err("Some went wrong");

    println!("{ok:?}");
    println!("{some_err:?}");

    let text = "50";
    //let result: Result<i32, std::num::ParseIntError> = text.parse::<i32>();
    let result = text.parse::<i32>();

    let val = match result {
        Ok(val) => val,
        Err(err) => {
            print!("err: {err}");
            0
        }
    };
    println!("val: {val}");

    let result = divide(3., 0.);
    println!("{}", result.is_ok());
    println!("{}", result.is_err());
    println!("unwrap_or: {:?}", result.unwrap_or(0.));
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0. {
        Err("Can no divide by 0".to_string())
    } else {
        Ok(numerator / denominator)
    }
}
