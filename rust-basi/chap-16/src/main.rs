fn main() {
    let clear_text = "Hello, Rustaceans!";

    println!("Clear text: {}", clear_text);

    for (index, c) in clear_text.chars().enumerate() {
        if index % 2 == 0 {
            print!("{}", c);
        }
    }
    print!("\n");

    for (index, c) in clear_text.chars().enumerate() {
        if index % 2 != 0 {
            print!("{}", c);
        }
    }
    print!("\n");

    let mut my_second_iter = clear_text.chars();
    loop {
        match my_second_iter.next() {
            Some(c) => print!("{}", c),
            None => break,
        }
    }

    let salt: u8 = 4;
    let encrypted_text = my_encryptor(clear_text, salt);
    println!("Encrypted text: {}", encrypted_text);

    let decrypted_text = my_decryptor(&encrypted_text, salt);
    println!("Decrypted text: {}", decrypted_text);

    let mut my_vec: Vec<i32> = vec![10, 21, 34, 47, 50];

    println!("Original vector: {:?}", my_vec);
    for elemen in my_vec.iter_mut() {
        *elemen = *elemen + 2;
    }
    println!("Updated vector: {:?}", my_vec);

    let only_even: Vec<i32> = my_vec.iter().cloned().filter(|x| x % 2 == 0).collect();
    println!("Only even elements: {:?}", only_even);

    let contains_only_even = my_vec.iter().all(|x| x % 2 == 0);
    println!("Contains only even elements: {}", contains_only_even);

    println!("Contains 't': {}", clear_text.chars().any(|c| c == 't'));
    println!("Contains 's': {}", clear_text.chars().any(|c| c == 's'));
}

fn my_encryptor(text: &str, salt: u8) -> String {
    let mut encrypted = String::new();
    for byte in text.bytes() {
        let encrypted_byte = byte ^ salt;
        encrypted.push(encrypted_byte as char);
    }
    encrypted
}

fn my_decryptor(encrypted_text: &str, salt: u8) -> String {
    let mut decrypted = String::new();
    for byte in encrypted_text.bytes() {
        let decrypted_byte = byte ^ salt;
        decrypted.push(decrypted_byte as char);
    }
    decrypted
}
