pub struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }

    pub fn greet(&self) -> String {
        format!(
            "Hello, my name is {} and I am {} years old.",
            self.name, self.age,
        )
    }
}
