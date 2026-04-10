fn main() {
    let num = 5;

    match num {
        1 | 3 | 5 | 7 => println!("Number {num} is odd"),
        2 | 4 | 6 | 8 => println!("Number {num} is even"),
        _ => println!("Out of reange"),
    }

    let result = match num {
        1 | 3 | 5 | 7 => {
            println!("Number {num} is odd");
            1
        }
        2 | 4 | 6 | 8 => {
            println!("Number {num} is even");
            2
        }
        _ => {
            println!("Out of reange");
            0
        }
    };
    println!("result: {result}");

    match num {
        val if val % 2 == 0 => println!("Number is even"),
        val if val % 2 != 0 => println!("Number is odd"),
        _ => unreachable!(),
    }
}
