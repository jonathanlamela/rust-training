use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Indovina il numero!");

    //Genera un numero casuale da 1 e 100
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Inserisci un numero");

        //Stringa che conterrà l'input dell'utente
        let mut guess = String::new();

        //Legge l'input dell'utente
        io::stdin()
            .read_line(&mut guess)
            .expect("Errore nella lettura dell'input");

        //Informa l'utente del numero inserito
        println!("Hai inserito: {}", guess);

        //Converte l'input in un numero
        //Se l'input non è un numero valido, stampa un messaggio di errore
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Per favore, inserisci un numero valido.");
                return;
            }
        };

        //Confronta il numero inserito con il numero segreto
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Troppo basso!");
            }
            Ordering::Greater => println!("Troppo alto!"),
            Ordering::Equal => {
                println!("Hai indovinato!");
                break;
            }
        }
    }
}
