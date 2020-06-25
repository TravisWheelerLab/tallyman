/// A very simple hash set implementation that uses
/// linear probing to handle collisions. This is intended
/// to be extremely lightweight to improve performance.
pub struct Hash {
    container: Vec<u64>,
    hits: Vec<u16>,
    capacity: u64,
}

impl Hash {
    pub fn new(capacity: usize) -> Hash {
        Hash {
            container: vec![0; capacity],
            hits: vec![0; capacity],
            capacity: capacity as u64,
        }
    }

    /// Add the given value to the set.
    pub fn add(&mut self, value: u64) {
        let hv = value % self.capacity;

        // We may now cast hv to a usize because we're sure
        // that it is < self.size and will therefore fit.
        let hv_index = hv as usize;
        let mut probed_index = hv_index;

        // Find the next empty slot (this is the linear probing bit).
        while self.container[probed_index] != 0 {
            probed_index += 1;

            if probed_index >= self.capacity as usize {
                probed_index = 0;
            }

            if probed_index == hv_index {
                panic!("hash full");
            }
        }

        self.container[probed_index] = value;
    }

    /// Return the position in the insertion order for the
    /// given value, or `0` if the value is not present.
    pub fn contains(&self, value: u64) -> bool {
        let hv = value % self.capacity;

        // We may now cast hv to a usize because we're sure
        // that it is < self.size and will therefore fit.
        let hv_index = hv as usize;
        let mut probed_index = hv_index;

        while self.container[probed_index] != 0 {
            if self.container[probed_index] == value {
                return true;
            }
            probed_index += 1;

            if probed_index >= self.capacity as usize {
                probed_index = 0;
            }

            if probed_index == hv_index {
                return false;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use crate::hash::Hash;

    #[test]
    fn test_create_hash() {
        let hash = Hash::new(10);
        assert_eq!(hash.capacity, 10);
        assert_eq!(hash.container.len(), 10);
        for i in 0..10 {
            assert_eq!(hash.container[i], 0);
        }
    }

    #[test]
    fn test_add_to_hash() {
        let mut hash = Hash::new(10);
        hash.add(10);
        hash.add(11);

        assert_eq!(hash.container[0], 10);
        assert_eq!(hash.container[1], 11);
    }

    #[test]
    fn test_contains_value() {
        let mut hash = Hash::new(10);
        hash.container[0] = 10;
        hash.container[2] = 12;
        // Collision that had to probe
        hash.container[3] = 2;

        assert_eq!(hash.contains(10), true);
        assert_eq!(hash.contains(12), true);
        assert_eq!(hash.contains(2), true);

        assert_eq!(hash.contains(1), false);
        assert_eq!(hash.contains(11), false);
    }
}
