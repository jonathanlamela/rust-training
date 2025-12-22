use core::time;
use std::sync::mpsc;
use std::thread::{self};

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();
    let max_val: i32 = 10;

    println!("Waiting to receive...");

    thread::spawn(move || {
        thread::sleep(time::Duration::from_secs(5));
        tx.send(max_val * 2).unwrap();
    });

    // Waits until a value is received
    let received = rx.recv().unwrap();
    println!("Received: {}", received);
}
