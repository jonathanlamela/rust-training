// Definisce una struct User con due campi: firstname e lastname
struct User {
    firstname: String,
    lastname: String,
}

//Creare un metodo per la struct User
impl User {
    fn full_name(&self) -> String {
        format!("{} {}", self.firstname, self.lastname)
    }
}

struct Position(f64, f64);

fn main() {
    let user = User {
        firstname: String::from("John"),
        lastname: String::from("Doe"),
    };
    println!("User: {} {}", user.firstname, user.lastname);
    println!("Full Name: {}", user.full_name());

    //Use of shorthand
    let firstname = String::from("Jane");
    let lastname = String::from("Smith");
    let user2 = User {
        firstname,
        lastname,
    };
    println!("User2: {} {}", user2.firstname, user2.lastname);
    println!("Full Name: {}", user2.full_name());

    let user_position = Position(10.0, 20.0);
    println!("User Position: ({}, {})", user_position.0, user_position.1);
}
