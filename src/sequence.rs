pub struct Seq {
    pub identifier: String,
    pub length: usize,
    pub characters: [char; 256],
}

impl Seq {
    pub fn new() -> Seq {
        Seq {
            identifier: String::with_capacity(256),
            length: 0,
            characters: [255 as char; 256],
        }
    }

    /// Constructor for use in tests to make it easier to build a fake sequence.
    pub fn pre_filled(id: &str, chars: &str) -> Seq {
        let mut characters = [255 as char; 256];
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
