#[allow(dead_code)]
enum Country {
    Italia,
    Francia,
    Germania,
    Spagna,
    Portogallo,
}

fn main() {
    let current_country: Country = Country::Italia;

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

    if let Some(result) = test_function(10) {
        println!("The result is: {}", result);
    } else {
        println!("No result returned.");
    }
}

fn test_function(n: i32) -> Option<i32> {
    if n % 2 == 0 { Some(n / 2) } else { None }
}
