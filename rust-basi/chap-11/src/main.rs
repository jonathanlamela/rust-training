use crate::packages::config;

pub mod packages;

fn main() {
    // This value is stored on the heap
    let box1 = Box::new(5);

    // This value is stored on the stack
    let stack1: &i32 = &15;

    println!("Box value: {}", box1);
    println!("Stack value: {}", stack1);

    let person = config::Person::new("Alice", 30);
    println!("Person's name: {}", person.name());
    println!("Person's age: {}", person.age());
}
