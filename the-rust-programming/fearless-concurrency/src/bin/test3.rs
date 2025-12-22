use core::time;
use std::sync::mpsc;
use std::thread::{self};

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    println!("Waiting to receive...");

    thread::spawn(move || {
        let value = String::from("Hello Word");
        thread::sleep(time::Duration::from_secs(5));
        tx.send(value).unwrap();

        /*
        This does not compile because `value` has been moved
        when we sent it through the channel.
         */
        // println!("Sent value {} from thread.", value);
    });

    // Waits until a value is received
    let received = rx.recv().unwrap();
    println!("Received: {}", received);
}
