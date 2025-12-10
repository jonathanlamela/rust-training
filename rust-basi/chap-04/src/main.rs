fn main() {
    let y = 0;
    let z = 20;

    println!("Using for loop:");
    for number in y..=z {
        if number % 2 == 0 {
            println!("{} is even", number);
        } else {
            println!("{} is odd", number);
        }
    }

    let mut i = 0;
    println!("\nUsing while loop:");
    while i <= 20 {
        if i % 2 == 0 {
            println!("{} is even", i);
        } else {
            println!("{} is odd", i);
        }
        i += 1;
    }
}
