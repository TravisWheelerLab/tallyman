/// A very simple hash set implementation that uses
/// linear probing to handle collisions. This is intended
/// to be extremely lightweight to improve performance.
#[derive(Clone, Debug)]
pub struct Hash {
    pub(crate) container: Vec<u64>,
    pub(crate) hits: Vec<u16>,
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
        dbg!(&value);
        let hv = value % self.capacity;
        dbg!(&self.capacity);
        dbg!(&hv);

        // We may now cast hv to a usize because we're sure
        // that it is < self.size and will therefore fit.
        let hv_index = hv as usize;
        let mut probed_index = hv_index;
        dbg!(&probed_index);
        dbg!(&self.container);
        dbg!(&self.container[probed_index]);

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

    pub fn get_index(&mut self, value: u64) -> usize {
        let hv = value % self.capacity;
        let hv_index = hv as usize;
        let mut probed_index = hv_index;

        // return if it's in the index calculated
        if self.container[probed_index] == value {
            return probed_index;
        }
        //otherwise we need to linear probe until
        //the DCE is found at subsequent indices
        else {
            while self.container[probed_index] != 0 {
                //loop to increment index, looking for the
                //index that actually contains the given DCE
                probed_index += 1;

                if probed_index >= self.capacity as usize {
                    probed_index = 0;
                }

                if self.container[probed_index] == value {
                    return probed_index;
                }
            }
        }
        return probed_index;
    }

    pub fn inc_hits(&mut self, value: u64) {
        let hv = value % self.capacity;
        let hv_index = hv as usize;
        let mut probed_index = hv_index;

        if self.container[probed_index] == value {
            self.hits[probed_index] += 1;
        } else {
            // Linear probing
            while self.container[probed_index] != 0 {
                probed_index += 1;

                if probed_index >= self.capacity as usize {
                    probed_index = 0;
                }
                //If we are at an index that matches the DCE we're looking
                //for, then increment the hits array at that index and stop
                if self.container[probed_index] == value {
                    self.hits[probed_index] += 1;
                    break;
                }
            }
        }
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
