use self::traits::{HexConverter, U32Converter, VectorConverter};

pub mod traits;

fn main() {
    let number: u32 = 182;
    let binary_representation: Vec<u8> = number.to_binary();

    let string_representation: String = binary_representation
        .iter()
        .rev()
        .map(|bit| bit.to_string())
        .collect();

    println!(
        "The binary representation of {} is {}",
        number, string_representation
    );

    let reconstructed_number: u32 = binary_representation.to_u32();
    println!(
        "The reconstructed number from binary representation is {}",
        reconstructed_number
    );

    let hex_representation: String = number.to_hex();
    println!(
        "The hexadecimal representation of {} is {}",
        number, hex_representation
    );

    let reconstructed_from_hex: u32 = hex_representation.to_u32();
    println!(
        "The reconstructed number from hexadecimal representation is {}",
        reconstructed_from_hex
    );
}
