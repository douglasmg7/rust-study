use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx_2 = tx.clone();

    // First thread.
    thread::spawn(move || {
        let vals = vec![
            String::from("21"),
            String::from("22"),
            String::from("23"),
            String::from("24"),
        ];
        for val in vals {
            tx_2.send(val).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    // Second thread.
    thread::spawn(move || {
        let vals = vec![
            String::from("11"),
            String::from("12"),
            String::from("13"),
            String::from("14"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    // let received = rx.recv().unwrap();
    for received in rx {
        println!("Received message: {received}");
    }
}
