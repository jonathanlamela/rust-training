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

    if let Country::Italia = current_country {
        println!("Benvenuti in Italia!");
    }

    if let Country::Francia = current_country {
        println!("Bienvenue en France!");
    }

    if let Country::Germania = current_country {
        println!("Willkommen in Deutschland!");
    }

    if let Country::Spagna = current_country {
        println!("¡Bienvenido a España!");
    }

    if let Country::Portogallo = current_country {
        println!("Bem-vindo a Portugal!");
    }
}
