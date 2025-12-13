use garden::vegetables::Asparagus;
use grocery::meat::Pork;

pub mod garden;
pub mod grocery;

fn main() {
    let asparagus = Asparagus { height: 30 };
    println!("Asparagus height: {}", asparagus.height);

    let asparagus2 = Asparagus::new(35);
    println!("Asparagus height: {}", asparagus2.height);

    let pork = Pork::new(10);
    println!("Pork weight: {}", pork.weight);
}
