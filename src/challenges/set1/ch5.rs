use std::{
    fs::{self, File},
    io::{self, BufRead, Error, ErrorKind, Result},
};

use super::{
    ch2::xor_buffers,
    ch3::{estimate_english_text_score, find_best_decrypt_result},
};

pub fn encrypt_with_repeated_key_xor(input: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {

    let mut iter = key.iter().cycle();
    let result = input.iter().map(|b| {
        let k = iter.next().unwrap();
        println!("Input byte: {}, Key byte: {}, XOR result: {}", b, k, b ^ k);
        b ^ k
    }).collect();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set1_challenge5() {
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = "ICE";

        let result = encrypt_with_repeated_key_xor(&input.into(), &key.into());

        assert_eq!(
            hex::encode(result),
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f",
            "Wrong answer."
        );
    }
}
