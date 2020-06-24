use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::constants::BUFFER_SIZE;
use crate::sequence::Seq;

pub struct SeqLoader<T: BufRead> {
    source_file: T,
    read_buffer: [u8; BUFFER_SIZE],
    read_index: usize,
    read_length: usize,
    line_buffer: [u8; BUFFER_SIZE],
    line_length: usize,
}

impl SeqLoader<BufReader<File>> {
    pub fn from_path(path: &Path) -> SeqLoader<BufReader<File>> {
        let file = File::open(path).unwrap();
        let mut loader = SeqLoader {
            source_file: BufReader::new(file),
            read_buffer: [0u8; BUFFER_SIZE],
            read_index: 0,
            read_length: 0,
            line_buffer: [0u8; BUFFER_SIZE],
            line_length: 0,
        };
        loader.next_line();
        loader
    }
}

impl<T: BufRead> SeqLoader<T> {
    pub fn from_bufread(file: T) -> SeqLoader<T> {
        let mut loader = SeqLoader {
            source_file: file,
            read_buffer: [0u8; BUFFER_SIZE],
            read_index: 0,
            read_length: 0,
            line_buffer: [0u8; BUFFER_SIZE],
            line_length: 0,
        };
        loader.next_line();
        loader
    }
}

impl<T: BufRead> SeqLoader<T> {
    /// Read into the read buffer, whether the
    /// data comes from the overflow or the file.
    fn fill_buffer(&mut self) {
        if self.read_index >= self.read_length {
            // There is nothing left over from last time, so
            // we can go ahead and read from the file again.
            self.read_length = self.source_file.read(&mut self.read_buffer).unwrap();
            self.read_index = 0;
        }
    }

    /// Reads the entire next line of the file and places
    /// it in the given buffer, not including the newline.
    /// The return value is the length of the line.
    fn next_line(&mut self) {
        self.line_length = 0;

        loop {
            self.fill_buffer();

            if self.read_length == 0 {
                return;
            }

            while self.read_index < self.read_length {
                if self.read_buffer[self.read_index] == '\n' as u8 {
                    self.read_index += 1;
                    return;
                }

                self.line_buffer[self.line_length] = self.read_buffer[self.read_index];
                self.read_index += 1;
                self.line_length += 1;
            }
        }
    }

    /// Read a single Seq from the file and store it in the given instance.
    pub fn next_seq(&mut self, seq: &mut Seq) -> bool {
        seq.identifier.clear();

        if self.line_length == 0 {
            // We didn't have any more data and we weren't
            // able to fill the Seq, so return false. In the
            // future we may want to be clearer about the error.
            return false;
        }

        if self.line_buffer[0] as char != '>' {
            // We expected to find an identifier
            // here, but we didn't. Therefore we bail.
            return false;
        }

        for i in 1..self.line_length {
            seq.identifier.push(self.line_buffer[i] as char);
        }

        let mut sequence_length = 0;

        // Run until we have a new sequence or have determined
        // that we don't have another valid sequence.
        loop {
            self.next_line();

            if self.line_length == 0 {
                // We hit the EOF.
                if sequence_length == 0 {
                    return false;
                }

                break;
            }

            if self.line_buffer[0] as char == '>' {
                // We hit the next identifier.
                if sequence_length == 0 {
                    return false;
                }

                break;
            }

            for i in 0..self.line_length {
                let c = self.line_buffer[i];
                seq.characters[i + sequence_length] = if c >= 97 && c <= 122 {
                    (c - 32) as char
                } else {
                    c as char
                };
            }
            sequence_length += self.line_length;
        }

        seq.length = sequence_length;

        true
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::fasta_read::SeqLoader;
    use crate::sequence::Seq;

    #[test]
    fn test_next_line() {
        let file = Cursor::new(String::from(">foo\nabcd\n>bar\ndcba"));
        let mut loader = SeqLoader::from_bufread(file);

        assert_eq!(loader.line_length, 4);
        assert_eq!(loader.line_buffer[1] as char, 'f');

        loader.next_line();
        assert_eq!(loader.line_length, 4);
        assert_eq!(loader.line_buffer[0] as char, 'a');

        loader.next_line();
        assert_eq!(loader.line_length, 4);
        assert_eq!(loader.line_buffer[1] as char, 'b');

        loader.next_line();
        assert_eq!(loader.line_length, 4);
        assert_eq!(loader.line_buffer[0] as char, 'd');

        loader.next_line();
        assert_eq!(loader.line_length, 0);
    }

    #[test]
    fn test_read_simple() {
        let file = Cursor::new(String::from(">foo\nabcd\n>bar\ndcba"));
        let mut loader = SeqLoader::from_bufread(file);
        let seq = &mut Seq::new();

        assert_eq!(seq.identifier, "");
        assert_eq!(
            seq.characters[0..seq.length],
            Vec::<char>::new()[0..seq.length]
        );

        assert_eq!(loader.next_seq(seq), true);
        assert_eq!(seq.identifier, "foo");
        assert_eq!(
            seq.characters[0..seq.length],
            "ABCD".chars().collect::<Vec<char>>()[0..seq.length]
        );

        assert_eq!(loader.next_seq(seq), true);
        assert_eq!(seq.identifier, "bar");
        assert_eq!(
            seq.characters[0..seq.length],
            "DCBA".chars().collect::<Vec<char>>()[0..seq.length]
        );

        assert_eq!(loader.next_seq(seq), false);
    }
}
