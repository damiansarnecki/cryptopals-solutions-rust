use std::io::{Error, ErrorKind, Result};

use super::ch2::xor_buffers;

pub fn decrypt(input: &Vec<u8>, key: u8) -> Vec<u8> {
    let cipher_buffer = vec![key; input.len()];
    let output = xor_buffers(input, &cipher_buffer).unwrap();

    output
}

pub fn find_best_decrypt_result(input : Vec<u8>) -> Vec<u8> {

    let mut best_score = 0;
    let mut best_message = Vec::new();


    for key in 0u8..255 {
        let decrypted_guess = decrypt(&input, key);
        let propability = estimate_english_text_score(&decrypted_guess);
        
        if propability > best_score {
            best_score = propability;
            best_message = decrypted_guess;
        }
    }

    return best_message
}

fn estimate_english_text_score(input : &Vec<u8>) -> i32 {

    let mut score : i32 = 0;

    for byte in input {
        match *byte as char {
            'e' => score += 13,
            't' => score += 9,
            'a' => score += 8,
            'o' => score += 8,
            'i' => score += 7,
            'n' => score += 7,
            's' => score += 6,
            'h' => score += 6,
            'r' => score += 6,
            'd' => score += 4,
            'l' => score += 4,
            'c' => score += 3,
            'u' => score += 3,
            'm' => score += 2,
            'w' => score += 2,
            'f' => score += 2,
            'g' => score += 2,
            'y' => score += 2,
            'p' => score += 2,
            'b' => score += 1,
            'v' => score += 1,
            'k' => score += 1,
            'x' | 'j' | 'q' | 'z' => score += 1,
            ' ' => score += 5,
            _ => score -= 5 
        }
    }
    
    score
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_challenge3() {
        let input_hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

        let best_message = find_best_decrypt_result(hex::decode(input_hex).unwrap());

        assert_eq!(String::from_utf8(best_message).unwrap(), "Cooking MC's like a pound of bacon", "Failed to parse hex to base 64");
    }
}
