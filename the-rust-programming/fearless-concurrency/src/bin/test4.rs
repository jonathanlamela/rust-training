use core::time;
use std::sync::mpsc;
use std::thread::{self};

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();

    println!("Waiting to receive...");

    thread::spawn(move || {
        for i in 1..5 {
            thread::sleep(time::Duration::from_secs(1));
            tx.send(i).unwrap();
        }
    });

    for value in rx {
        println!("Received: {}", value);
    }

    println!("All values received.");
}
