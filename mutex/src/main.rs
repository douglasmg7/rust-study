use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for i in 1..=10 {
        let counter = Arc::clone(&counter);
        let handler = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Thread {i} proceded.");
        });
        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    println!("result: {}", *counter.lock().unwrap());
}
