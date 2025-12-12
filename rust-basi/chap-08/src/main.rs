struct People {
    first_name: String,
    last_name: String,
    age: u8,
}

fn main() {
    let people1: (&str, &str, u8) = ("Alice", "Smith", 30);
    let people2: (&str, &str, u8) = ("Bob", "Johnson", 25);

    println!("Person 1: {} {}, Age: {}", people1.0, people1.1, people1.2);
    println!("Person 2: {} {}, Age: {}", people2.0, people2.1, people2.2);

    let person1 = People {
        first_name: String::from("Alice"),
        last_name: String::from("Smith"),
        age: 30,
    };
    let person2 = People {
        first_name: String::from("Bob"),
        last_name: String::from("Johnson"),
        age: 25,
    };

    println!(
        "Person 1: {} {}, Age: {}",
        person1.first_name, person1.last_name, person1.age
    );
    println!(
        "Person 2: {} {}, Age: {}",
        person2.first_name, person2.last_name, person2.age
    );
}
