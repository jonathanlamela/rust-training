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
}

fn convert_to_binary(n: u32) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    if n == 0 {
        result.push(0);
        return result;
    }
    let mut num: u32 = n;

    while num > 0 {
        if num % 2 == 1 {
            result.push(1);
        } else {
            result.push(0);
        }
        num /= 2;
    }
    result
}
