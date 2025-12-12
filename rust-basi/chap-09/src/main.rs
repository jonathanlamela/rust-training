#[allow(dead_code)]
enum Country {
    Italy,
    France,
    Spain,
}

fn main() {
    let country1 = Country::France;
    let mut number = 5;

    println!("Number before multiply_by_two: {}", number);
    multiply_by_two(&mut number);
    println!("Number after multiply_by_two: {}", number);

    match country1 {
        Country::Italy => italy_capital(),
        Country::France => france_capital(),
        Country::Spain => spain_capital(),
    }

    let passed_number = 12;

    println!("Passed number: {}", passed_number);
    number_receiver(passed_number);
    println!("Passed number: {}", passed_number);

    let passed_string = String::from("Hello, world!");
    println!("Passed string: {}", passed_string);
    string_receiver(&passed_string);
    println!("Passed string: {}", &passed_string);
}

fn number_receiver(x: i32) {
    println!("Received number: {}", x);
}

fn string_receiver(s: &String) {
    println!("Received string: {}", s);
}

fn italy_capital() {
    fn show_message() {
        println!("La capitale dell'Italia è Roma");
    }
    show_message()
}

fn france_capital() {
    fn show_message() {
        println!("La capitale de la France est Paris");
    }
    show_message()
}

fn spain_capital() {
    fn show_message() {
        println!("La capitale de l'Espagne est Madrid");
    }
    show_message()
}

fn multiply_by_two(x: &mut i32) {
    *x = *x * 2;
}
