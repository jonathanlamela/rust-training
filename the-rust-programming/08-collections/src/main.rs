use std::collections::HashMap;

fn main() {
    //Creation of a vectors
    let v: Vec<i32> = Vec::new(); // Immutable vector
    println!("v: {:?}", v);

    let mut v2 = vec![1, 2, 3]; // Mutable vector
    v2.push(4); // Adding an element to the mutable vector
    println!("v2: {:?}", v2);

    // Trying to access an element by index
    if let Some(first) = v.get(0) {
        println!("First element: {}", first);
    } else {
        println!("No first element found.");
    }

    let fifth: Option<&i32> = v2.get(4);
    match fifth {
        Some(value) => println!("Fifth element: {}", value),
        None => println!("No fifth element found."),
    }

    //HashMap creation and usage
    let mut scores = HashMap::new();
    scores.insert("Alice", 50);
    scores.insert("Bob", 70);

    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
}
