fn main() {
    let mut i = 0;
    loop {
        i += 1;
        if i % 2 == 0 {
            println!("{i}")
        } else {
            continue;
        };
        println!("Test break");
        if i > 4 {
            break;
        }
    }

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}