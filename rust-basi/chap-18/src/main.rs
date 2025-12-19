pub mod implementations;
pub mod structs;

fn main() {
    let p1 = structs::Person::new(String::from("Alice"), 30);
    let p2 = structs::Person::new(String::from("Bob"), 25);

    println!("{}", p1.greet());
    println!("{}", p2.greet());
}
