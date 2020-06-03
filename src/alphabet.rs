use std::collections::HashMap;

pub const ALPHABET: [char; 4] = ['A', 'C', 'G', 'T'];

// TODO: Handle lowercase letters
pub fn encode_char(c: char) -> u64 {
    match ALPHABET.binary_search(&c) {
        Ok(i) => i as u64,
        Err(_) => panic!(format!("invalid character '{}'", c)),
    }
}
