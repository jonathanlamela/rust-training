use std::io::Write;

fn main() {
    /*
    let args_receved = std::env::args();

    for argument in args_receved.into_iter().skip(1) {
        println!("Argument: {}", argument);
    }
     */

    println!("Type something: ");
    let mut row_received = String::new();
    std::io::stdin()
        .read_line(&mut row_received)
        .expect("Failed to read line");
    println!("You typed: {}", row_received.trim());

    println!("Type a number:");
    let mut number_received = String::new();
    std::io::stdin()
        .read_line(&mut number_received)
        .expect("Failed to read line");
    let number: i32 = number_received
        .trim()
        .parse()
        .expect("Please type a valid number");
    println!("You typed the number: {}", number);

    let mut file_out = std::fs::File::create("output.txt").expect("Failed to create file");

    for i in 1..=100 {
        writeln!(file_out, "This is line number {}", i).expect("Failed to write to file");
    }

    let mut file_in =
        std::fs::File::open("output.txt").expect("Failed to open the file for reading");
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file_in, &mut contents).expect("Failed to read the file");
    println!("File contents:\n{}", contents);
}
