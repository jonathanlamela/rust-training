fn main() {
    let number: u32 = 182;
    let binary_representation: Vec<u8> = u32_to_binary(number);

    let string_representation: String = binary_representation
        .iter()
        .rev()
        .map(|bit| bit.to_string())
        .collect();

    println!(
        "The binary representation of {} is {}",
        number, string_representation
    );

    let reconstructed_number: u32 = binary_to_u32(&binary_representation);
    println!(
        "The reconstructed number from binary representation is {}",
        reconstructed_number
    );

    let hex_representation: String = u32_to_hex(number);
    println!(
        "The hexadecimal representation of {} is {}",
        number, hex_representation
    );

    let reconstructed_from_hex: u32 = hex_to_u32(&hex_representation);
    println!(
        "The reconstructed number from hexadecimal representation is {}",
        reconstructed_from_hex
    );
}

fn u32_to_binary(n: u32) -> Vec<u8> {
    // create a vector to hold the binary digits
    let mut result: Vec<u8> = vec![];

    // handle the case when n is 0
    if n == 0 {
        // binary representation of 0 is just 0
        result.push(0);
        return result;
    }

    // convert n to binary
    let mut num: u32 = n;

    // repeatedly divide by 2 and store the remainder
    while num > 0 {
        result.push((num % 2) as u8);
        num /= 2;
    }
    result
}

fn binary_to_u32(bits: &[u8]) -> u32 {
    // convert binary representation back to u32
    let mut result: u32 = 0;

    // iterate over the bits and calculate the corresponding value
    for (i, &bit) in bits.iter().enumerate() {
        // if the bit is 1, add the corresponding power of 2
        if bit == 1 {
            // 2 raised to the power of i
            result += 2u32.pow(i as u32);
        }
    }
    result
}

fn u32_to_hex(n: u32) -> String {
    // create a string to hold the hexadecimal representation
    let mut result = String::new();

    // hexadecimal characters
    let hex_chars = "0123456789ABCDEF".chars().collect::<Vec<char>>();

    let mut num = n;

    // handle the case when n is 0
    if num == 0 {
        result.push('0');
        return result;
    }

    while num > 0 {
        // get the remainder when divided by 16
        let remainder = (num % 16) as usize;

        // prepend the corresponding hex character
        result.push(hex_chars[remainder]);

        // divide the number by 16 for the next iteration
        num /= 16;
    }

    // reverse the string to get the correct order
    result.chars().rev().collect()
}

fn hex_to_u32(hex: &str) -> u32 {
    // hexadecimal characters
    let hex_chars = "0123456789ABCDEF";
    let mut result: u32 = 0;

    // iterate over the hex string in reverse order
    for (i, c) in hex.chars().rev().enumerate() {
        // find the position of the character in the hex_chars string
        if let Some(pos) = hex_chars.find(c) {
            result += (pos as u32) * 16u32.pow(i as u32);
        } else {
            panic!("Invalid hexadecimal character: {}", c);
        }
    }
    result
}
