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
    println!("Length of the string: {}", io_sono_una_slice.len());
    println!("Capacity of the string: {}", io_sono_una_slice.capacity());
    println!("Is the string empty? {}", io_sono_una_slice.is_empty());
    println!("First 4 characters: {}", &io_sono_una_slice[0..4]);

    let my_var = 2;
    {
        let mut my_second_var = my_var;
        my_second_var += 3;
        println!("Inner my_var: {}", my_second_var);
    }
    println!("Outer my_var: {}", my_var);

    let my_string = String::from("Hello, world!");
    {
        let my_borrowed_string = &my_string;
        println!("Borrowed string: {}", my_borrowed_string);
    }
    println!("Original string: {}", my_string);
}

// s is passed as ownership so it is no longer accessible afterwards
#[allow(dead_code)]
fn prendi_possesso(s: String) {
    println!("Prendo possesso di: {}", s);
}

// s is passed as a reference, so it is still accessible afterwards
fn prendi_prestito(s: &String) {
    println!("I am borrowing: {}", s);
}
