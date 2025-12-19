fn main() {
    println!("Hello, world!");

    let int_value: i32 = 42;
    let float_value: f64 = 3.14;

    println!("The type of int_value is: {}", int_value.get_type_name());
    println!(
        "The type of float_value is: {}",
        float_value.get_type_name()
    );

    println!("Double of int_value is: {}", int_value.double_value());
    println!("Double of float_value is: {}", float_value.double_value());
}

trait MyNumber {
    fn get_type_name(&self) -> &'static str;
    fn double_value(&self) -> Self
    where
        Self: Sized;
}

impl MyNumber for i32 {
    fn get_type_name(&self) -> &'static str {
        "i32"
    }
    fn double_value(&self) -> Self {
        self * 2
    }
}

impl MyNumber for f64 {
    fn get_type_name(&self) -> &'static str {
        "f64"
    }
    fn double_value(&self) -> Self {
        self * 2.0
    }
}
