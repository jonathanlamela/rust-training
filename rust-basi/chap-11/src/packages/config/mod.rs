pub struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn new(name: &str, age: u8) -> Person {
        Person {
            name: String::from(name),
            age,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn age(&self) -> u8 {
        self.age
    }
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("Dropping Person: {}", self.name);
    }
}
