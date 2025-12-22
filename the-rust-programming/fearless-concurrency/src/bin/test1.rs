use core::time;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};

fn my_handler(sender: &mpsc::Sender<i32>) -> JoinHandle<()> {
    let max_val = 10;
    let sender = sender.clone();

    thread::sleep(time::Duration::from_secs(2));
    thread::spawn(move || sender.send(max_val * 2).unwrap())
}

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();

    my_handler(&tx);

    let received = rx.recv().unwrap();
    println!("Received: {}", received);

    tx.send(60).unwrap();

    let received = rx.recv().unwrap();
    println!("Received: {}", received);

    println!("All threads have completed.");
}
