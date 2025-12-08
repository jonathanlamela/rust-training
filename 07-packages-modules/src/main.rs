use garden::vegetables;
use grocery::meat;

pub mod garden;
pub mod grocery;

fn main() {
    let asparagus = vegetables::Asparagus { height: 30 };
    println!("Asparagus height: {}", asparagus.height);

    let asparagus2 = vegetables::Asparagus::new(35);
    println!("Asparagus height: {}", asparagus2.height);

    let pork = meat::Pork::new(10);
    println!("Pork weight: {}", pork.weight);
}
