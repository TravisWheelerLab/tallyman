use crate::constants::BUFFER_SIZE;

#[derive(Debug)]
pub struct Seq {
    pub identifier: String,
    pub length: usize,
    pub characters: [char; BUFFER_SIZE],
}

impl Seq {
    pub fn new() -> Seq {
        Seq {
            identifier: String::with_capacity(BUFFER_SIZE),
            length: 0,
            characters: [255 as char; BUFFER_SIZE],
        }
    }

    /// Constructor for use in tests to make it easier to build a fake sequence.
    pub fn pre_filled(id: &str, chars: &str) -> Seq {
        let mut characters = [255 as char; BUFFER_SIZE];
        let chars_indexed: Vec<char> = chars.chars().collect();
        for i in 0..chars.len() {
            characters[i] = chars_indexed[i];
        }

        Seq {
            identifier: String::from(id),
            length: chars.len(),
            characters,
        }
    }
}
