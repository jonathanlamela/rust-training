fn main() {
    // Use of String
    let mut message = String::from("Hello");
    message.push_str(", world!");
    println!("{}", message);

    let x1 = String::from("Hello");
    let x2 = x1.clone(); // Cloning x1 to x2
    println!("x1: {}, x2: {}", x1, x2);

    let s1 = String::from("Hello");
    prendi_prestito(&s1);

    let mut ioSonoUnaSlice = String::from("Hello");
    ioSonoUnaSlice.push_str(", world!");
    println!("{}", ioSonoUnaSlice);
    println!("Lunghezza della stringa: {}", ioSonoUnaSlice.len());
    println!("Capacità della stringa: {}", ioSonoUnaSlice.capacity());
    println!("La stringa è vuota? {}", ioSonoUnaSlice.is_empty());
    println!("Primi 4 caratteri: {}", &ioSonoUnaSlice[0..4]);
}

// s viene postato come ownership quindi dopo non è accessibile
fn prendi_possesso(s: String) {
    println!("Prendo possesso di: {}", s);
}

// s viene passato come riferimento, quindi dopo è ancora accessibile
fn prendi_prestito(s: &String) {
    println!("Prendo in prestito: {}", s);
}
