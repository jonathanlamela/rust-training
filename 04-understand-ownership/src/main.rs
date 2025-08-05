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

    let mut io_sono_una_slice = String::from("Hello");
    io_sono_una_slice.push_str(", world!");
    println!("{}", io_sono_una_slice);
    println!("Lunghezza della stringa: {}", io_sono_una_slice.len());
    println!("Capacità della stringa: {}", io_sono_una_slice.capacity());
    println!("La stringa è vuota? {}", io_sono_una_slice.is_empty());
    println!("Primi 4 caratteri: {}", &io_sono_una_slice[0..4]);
}

// s viene postato come ownership quindi dopo non è accessibile
#[allow(dead_code)]
fn prendi_possesso(s: String) {
    println!("Prendo possesso di: {}", s);
}

// s viene passato come riferimento, quindi dopo è ancora accessibile
fn prendi_prestito(s: &String) {
    println!("Prendo in prestito: {}", s);
}
