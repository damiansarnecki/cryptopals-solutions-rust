use std::{fs::{self, File}, io::{self, BufRead, Error, ErrorKind, Result}};

use super::{ch2::xor_buffers, ch3::{estimate_english_text_score, find_best_decrypt_result}};

pub fn find_single_xor_encrypted_string_from_file(file: &str) ->  Vec<u8> {
    let file = File::open("challenge_files/ch4.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut best_candidate : Vec<u8> = Vec::new();
    let mut best_score = 0;

    for line in reader.lines() {
        let line = line.unwrap(); 
        let line_bin = hex::decode(line.trim_end()).unwrap();

        let decrypt_result = find_best_decrypt_result(line_bin);
        let result_score = estimate_english_text_score(&decrypt_result);

        if result_score > best_score {
            best_candidate = decrypt_result;
            best_score = result_score;
        }
    }

    best_candidate
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set1_challenge4() {

        let result = find_single_xor_encrypted_string_from_file("./challenge_files/ch4.txt");

        assert_eq!(String::from_utf8(result).unwrap().trim(), "Now that the party is jumping", "Wrong answer.");
    }
}
