fn main() {
    let mut color_num = color_to_number("blue");
    println!("Color num: {color_num}");
    color_num = color_to_number_match("green");
    println!("Color num: {color_num}");
    let mut fac_num = factorial(5);
    println!("Factorial 5: {fac_num}");
    fac_num = factorial_list(4);
    println!("Factorial 4: {fac_num}");

    fn color_to_number(color: &str) -> i32 {
        if color == "red" {
            1
        } else if color == "green" {
            2
        } else if color == "blue" {
            3
        } else {
            0
        }
    }

    fn color_to_number_match(color: &str) -> i32 {
        match color {
            "red" => 1,
            "green" => 2,
            "blue" => 3,
            _ => 0,
        }
    }

    fn factorial(num: i32) -> i32 {
        if num == 1 {
            return num;
        }

        num * factorial(num - 1)
    }

    fn factorial_list(num: i32) -> i32 {
        let mut result = 1;

        for i in 1..=num {
            result *= i;
        }
        result
    }
}
