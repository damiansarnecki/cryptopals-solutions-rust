use base64::{engine::general_purpose, Engine};

pub fn hex_to_base64(hex_str: &str) -> String {
    let encoded_hex = hex::decode(hex_str).unwrap();
    general_purpose::STANDARD.encode(encoded_hex)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set1_challenge1() {

        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected_output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        

        let result = hex_to_base64(input);

        assert_eq!(result, expected_output, "Wrong answer");
    }
}