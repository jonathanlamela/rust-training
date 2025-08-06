use std::fs::File;

fn main() {
    let file = File::open("../data/test1.txt");

    match file {
        Ok(_) => println!("File opened successfully!"),
        Err(e) => {
            panic!("Failed to open file: {}", e);
        }
    }
}
