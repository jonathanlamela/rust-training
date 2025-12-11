use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    //Generate a random number between 1 and 100
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Enter a number");

        //String that will contain the user's input
        let mut guess = String::new();

        //Reads the user's input
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input");

        //Informs the user of the entered number
        println!("You entered: {}", guess);

        //Converts the input to a number
        //If the input is not a valid number, prints an error message
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                return;
            }
        };

        //Compares the entered number with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too low!");
            }
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
        }
    }
}
