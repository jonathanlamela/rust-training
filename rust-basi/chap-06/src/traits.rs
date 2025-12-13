pub trait U32Converter {
    fn to_binary(&self) -> Vec<u8>;
    fn to_hex(&self) -> String;
}

pub trait VectorConverter {
    fn to_u32(&self) -> u32;
}

pub trait HexConverter {
    fn to_u32(&self) -> u32;
}

impl HexConverter for String {
    fn to_u32(&self) -> u32 {
        // hexadecimal characters
        let hex_chars = "0123456789ABCDEF";
        let mut result: u32 = 0;

        // iterate over the hex string in reverse order
        for (i, c) in self.chars().rev().enumerate() {
            // find the position of the character in the hex_chars string
            if let Some(pos) = hex_chars.find(c) {
                result += (pos as u32) * 16u32.pow(i as u32);
            } else {
                panic!("Invalid hexadecimal character: {}", c);
            }
        }
        result
    }
}

impl U32Converter for u32 {
    fn to_binary(&self) -> Vec<u8> {
        // create a vector to hold the binary digits
        let mut result: Vec<u8> = vec![];

        // handle the case when n is 0
        if *self == 0 {
            // binary representation of 0 is just 0
            result.push(0);
            return result;
        }

        // convert n to binary
        let mut num: u32 = *self;

        // repeatedly divide by 2 and store the remainder
        while num > 0 {
            result.push((num % 2) as u8);
            num /= 2;
        }
        result
    }

    fn to_hex(&self) -> String {
        // create a string to hold the hexadecimal representation
        let mut result = String::new();

        // hexadecimal characters
        let hex_chars = "0123456789ABCDEF".chars().collect::<Vec<char>>();

        let mut num = *self;

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
}

impl VectorConverter for Vec<u8> {
    fn to_u32(&self) -> u32 {
        let mut result: u32 = 0;

        for (i, &bit) in self.iter().enumerate() {
            result += (bit as u32) * 2u32.pow(i as u32);
        }
        result
    }
}
