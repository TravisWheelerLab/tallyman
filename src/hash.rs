use anyhow::{bail, Result};

/// A very simple hash set implementation that uses
/// linear probing to handle collisions. This is intended
/// to be extremely lightweight to improve performance.
#[derive(Debug, Clone)]
pub struct Hash {
    pub(crate) key: Vec<u64>,
    pub(crate) hits: Vec<i16>,
    capacity: u64,
}

impl Hash {
    pub fn new(capacity: usize) -> Hash {
        Hash {
            key: vec![0; capacity],
            hits: vec![-1; capacity],
            capacity: capacity as u64,
        }
    }

    /// Add the given value to the set.
    /// There is a problem that the `value` legit could be 0
    /// in the case of a poly-A sequence, so we init the hits
    /// to -1 and check that hash for values that have been taken
    pub fn add(&mut self, value: u64) -> Result<()> {
        // We may now cast hv to a usize because we're sure
        // that it is < self.size and will therefore fit.
        let hv_index = (value % self.capacity) as usize;
        let mut probed_index = hv_index;

        // Find the next empty slot (this is the linear probing bit).
        // Hits start at -1 and get set to 0 when filled!
        while self.hits[probed_index] == 0 {
            probed_index += 1;

            if probed_index >= self.capacity as usize {
                probed_index = 0;
            }

            if probed_index == hv_index {
                bail!("hash full");
            }
        }

        self.key[probed_index] = value;
        self.hits[probed_index] = 0;
        Ok(())
    }

    /// This is too slow to use!
    /// Find `value` key
    pub fn find(&mut self, value: u64) -> Option<usize> {
        let start = (value % self.capacity) as usize;
        let mut probed_index = start;

        //if self.key[probed_index] == value {
        //    return Some(probed_index);
        //} else {
        //    while self.key[probed_index] != 0 {
        //        probed_index += 1;

        //        // We've gone off the end
        //        if probed_index >= self.capacity as usize {
        //            probed_index = 0;
        //        }

        //        // We've looped around to the beginning
        //        if probed_index == start {
        //            break;
        //        }

        //        if self.key[probed_index] == value {
        //            return Some(probed_index);
        //        }
        //    }
        //}

        loop {
            if self.key[probed_index] == value {
                return Some(probed_index);
            } else {
                probed_index += 1;

                // We've gone off the end
                if probed_index >= self.capacity as usize {
                    probed_index = 0;
                }

                // We've looped around to the beginning
                if probed_index == start {
                    break;
                }
            }
        }

        None
    }

    pub fn inc_hits(&mut self, value: u64) {
        let start = (value % self.capacity) as usize;
        let mut probed_index = start;

        if self.key[probed_index] == value {
            self.hits[probed_index] += 1;
        } else {
            // Linear probing
            while self.key[probed_index] != 0 {
                probed_index += 1;

                // We've gone off the end
                if probed_index >= self.capacity as usize {
                    probed_index = 0;
                }

                // We've looped around to the beginning
                if probed_index == start {
                    break;
                }

                //If we are at an index that matches the DCE we're looking
                //for, then increment the hits array at that index and stop
                if self.key[probed_index] == value {
                    self.hits[probed_index] += 1;
                    break;
                }
            }
        }
    }

    pub fn get_hits(&mut self, value: u64) -> Option<u64> {
        self.find(value)
            .and_then(|index| Some(self.hits[index] as u64))
    }
}

#[cfg(test)]
mod test {
    use crate::hash::Hash;

    #[test]
    fn hash_create() {
        let hash = Hash::new(10);
        assert_eq!(hash.capacity, 10);
        assert_eq!(hash.key.len(), 10);
        assert_eq!(hash.hits.len(), 10);

        for i in 0..10 {
            assert_eq!(hash.key[i], 0);
            assert_eq!(hash.hits[i], -1);
        }
    }

    #[test]
    fn hash_add() {
        let mut hash = Hash::new(10);
        let _ = hash.add(10);
        assert_eq!(hash.key[0], 10);
        assert_eq!(hash.hits[0], 0);

        let _ = hash.add(11);
        assert_eq!(hash.key[1], 11);
        assert_eq!(hash.hits[1], 0);

        // 0 will collide with 10 and should be pushed over 2 places
        let _ = hash.add(0);
        assert_eq!(hash.key[2], 0);
        assert_eq!(hash.hits[2], 0);

        // 19 should be placed at the end
        let _ = hash.add(19);
        assert_eq!(hash.key[9], 19);
        assert_eq!(hash.hits[9], 0);

        // 9 will conflict with 19 and will wrap around
        let _ = hash.add(9);
        assert_eq!(hash.key[3], 9);
        assert_eq!(hash.hits[3], 0);
    }

    #[test]
    fn hash_find() {
        let mut hash = Hash::new(10);
        for val in &[10, 11, 0] {
            let _ = hash.add(*val);
        }

        let res = hash.find(10);
        assert_eq!(res, Some(0));

        let res = hash.find(11);
        assert_eq!(res, Some(1));

        let res = hash.find(0);
        assert_eq!(res, Some(2));

        let res = hash.find(12);
        assert!(res.is_none());
    }

    #[test]
    fn hash_inc_get_hits() {
        let mut hash = Hash::new(10);
        for val in &[10, 11, 0] {
            let _ = hash.add(*val);
        }

        for val in &[10, 11, 10, 11, 11, 0] {
            let _ = hash.inc_hits(*val);
        }

        assert_eq!(hash.get_hits(10), Some(2));
        assert_eq!(hash.get_hits(11), Some(3));
        assert_eq!(hash.get_hits(0), Some(1));
        assert!(hash.get_hits(1).is_none());
    }
}
