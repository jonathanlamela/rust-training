fn main() {
    let number: u32 = 13;
    let binary_representation: Vec<u8> = convert_to_binary(number);

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

fn convert_to_binary(n: u32) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    if n == 0 {
        result.push(0);
        return result;
    }
    let mut num: u32 = n;

    while num > 0 {
        result.push((num % 2) as u8);
        num /= 2;
    }
    result
}

fn binary_to_u32(bits: &[u8]) -> u32 {
    let mut result: u32 = 0;
    for (i, &bit) in bits.iter().enumerate() {
        if bit == 1 {
            result += 2u32.pow(i as u32);
        }
    }
    result
}
