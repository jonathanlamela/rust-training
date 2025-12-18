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
}
