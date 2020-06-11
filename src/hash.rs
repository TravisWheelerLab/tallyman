/// A very simple hash set implementation that uses
/// linear probing to handle collisions. This is intended
/// to be extremely lightweight to improve performance.
pub struct Hash {
    container: Vec<u64>,
    occupied: Vec<usize>,
    capacity: usize,
    size: usize,
}

impl Hash {
    pub fn new(capacity: usize) -> Hash {
        Hash {
            container: vec![0; capacity],
            occupied: vec![0; capacity],
            capacity,
            size: 0,
        }
    }

    /// Add the given value to the set.
    pub fn add(&mut self, value: u64) {
        let hv = value % (self.capacity as u64);

        // We may now cast hv to a usize because we're sure
        // that it is < self.size and will therefore fit.
        let hv_index = hv as usize;
        let mut probed_index = hv_index;

        // Find the next empty slot (this is the linear probing bit).
        while self.occupied[probed_index] != 0 {
            probed_index += 1;

            if probed_index >= self.capacity {
                probed_index = 0;
            }

            if probed_index == hv_index {
                panic!("hash full");
            }
        }

        self.size += 1;
        self.container[probed_index] = value;
        self.occupied[probed_index] = self.size;
    }

    /// Return the position in the insertion order for the
    /// given value, or `0` if the value is not present.
    pub fn contains(&self, value: u64) -> usize {
        let hv = value % (self.capacity as u64);

        // We may now cast hv to a usize because we're sure
        // that it is < self.size and will therefore fit.
        let hv_index = hv as usize;
        let mut probed_index = hv_index;

        while self.occupied[probed_index] != 0 {
            if self.container[probed_index] == value {
                return self.occupied[probed_index];
            }
            probed_index += 1;

            if probed_index >= self.capacity {
                probed_index = 0;
            }

            if probed_index == hv_index {
                return 0;
            }
        }

        0
    }
}

#[cfg(test)]
mod test {
    use crate::hash::Hash;

    #[test]
    fn test_create_hash() {
        let hash = Hash::new(10);
        for i in 0..10 {
            assert_eq!(hash.container[i], 0);
            assert_eq!(hash.occupied[i], 0);
        }
    }

    #[test]
    fn test_add_to_hash() {
        let mut hash = Hash::new(10);
        hash.add(0);
        hash.add(1);

        assert_eq!(hash.container[0], 0);
        assert_eq!(hash.occupied[0], 1);

        assert_eq!(hash.container[1], 1);
        assert_eq!(hash.occupied[1], 2);
    }

    #[test]
    fn test_contains_value() {
        let mut hash = Hash::new(10);
        hash.occupied[0] = 1;
        hash.container[0] = 0;
        hash.occupied[2] = 2;
        hash.container[2] = 2;
        // Collision that had to probe
        hash.occupied[3] = 3;
        hash.container[3] = 12;

        assert_eq!(hash.contains(0), 1);
        assert_eq!(hash.contains(2), 2);
        assert_eq!(hash.contains(12), 3);

        assert_eq!(hash.contains(1), 0);
        assert_eq!(hash.contains(13), 0);
    }
}
