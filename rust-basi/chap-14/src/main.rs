fn main() {
    let firstname = String::from("Ferris");
    let lastname = String::from("Crab");
    let fullname = format!("{} {}", firstname, lastname);
    println!("Full name: {}", fullname);

    let s1 = "Hello";
    let s1_as_string = s1.to_string();
    let s2 = String::from("World");
    println!("s1 as String: {}", s1_as_string);
    println!("{} {}", s1, s2);
}
