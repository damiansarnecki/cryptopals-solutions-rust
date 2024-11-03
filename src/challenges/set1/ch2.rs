use std::io::{Error, ErrorKind, Result};

pub fn xor_buffers(b1: &Vec<u8>, b2: &Vec<u8>) -> Result<Vec<u8>> {
    if b1.len() == b2.len() {
        let xor_output: Vec<u8> = b1.iter().zip(b2.iter()).map(|(b1, b2)| b1 ^ b2).collect();
        Ok(xor_output)
    } else {
        Err(Error::new(ErrorKind::InvalidInput, "Buffers have mismatched lengths"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set1_challenge2() {
        let hex_one = "1c0111001f010100061a024b53535009181c";
        let hex_two = "686974207468652062756c6c277320657965";

        let output = xor_buffers(&hex::decode(hex_one).unwrap(), &hex::decode(hex_two).unwrap()).unwrap();
        let output_hex = hex::encode(output);

        let expected_output_hex = "746865206b696420646f6e277420706c6179";

        assert_eq!(output_hex, expected_output_hex, "Failed to parse hex to base 64");
    }
}
