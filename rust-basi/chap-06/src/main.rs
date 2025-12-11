fn main() {
    let number: u32 = 13;
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
