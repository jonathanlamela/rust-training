#[allow(dead_code)]
enum Country {
    Italia,
    Francia,
    Germania,
    Spagna,
    Portogallo,
}

fn main() {
    let current_country = Country::Italia;

    match current_country {
        Country::Italia => println!("You're in Italy!"),
        Country::Francia => println!("You're in France!"),
        Country::Germania => println!("You're in Germany!"),
        Country::Spagna => println!("You're in Spain!"),
        Country::Portogallo => println!("You're in Portugal!"),
    }
}
