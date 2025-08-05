use garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let asparagus = Asparagus { height: 30 };
    println!("Asparagus height: {}", asparagus.height);

    let asparagus2 = Asparagus::new(35);
    println!("Asparagus height: {}", asparagus2.height);
}
