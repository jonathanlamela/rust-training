pub mod person;

fn main() {
    let p1 = person::Person::new(String::from("Alice"), 30);
    let p2 = person::Person::new(String::from("Bob"), 25);

    println!("{}", p1.greet());
    println!("{}", p2.greet());
}
