/// A very simple hash set implementation that uses
/// linear probing to handle collisions. This is intended
/// to be extremely lightweight to improve performance.
#[derive(Clone)]
pub struct Hashmap {
    container: Vec<u64>,
    pub(crate) dce_id: Vec<Vec<String>>,
    capacity: u64,
}

impl Hashmap {
    pub fn new(capacity: usize) -> Hashmap {
        Hashmap {
            container: vec![0; capacity],
            dce_id: vec![vec![0.to_string(); 1]; capacity],
            capacity: capacity as u64,
        }
    }

    /// Add the given value to the set.
    pub fn add(&mut self, value: u64, id: String) {
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
        self.dce_id[probed_index] = id;
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

    pub fn get_index(&mut self, value: u64) -> usize {
        let hv = value % self.capacity;
        let hv_index = hv as usize;
        let mut probed_index = hv_index;

        // return if it's in the index calculated
        if self.container[probed_index] == value{
            return probed_index;
        }
        //otherwise we need to linear probe until
        //the DCE is found at subsequent indices
        else{
            while self.container[probed_index] != 0 {
                //loop to increment index, looking for the
                //index that actually contains the given DCE
                probed_index += 1;

                if probed_index >= self.capacity as usize {
                    probed_index = 0;
                }

                if self.container[probed_index] == value{
                    return probed_index;
                }
            }
        }
        return probed_index;
    }


}

#[cfg(test)]
mod test {
    use crate::hashmap::Hashmap;

    #[test]
    fn test_create_hash() {
        let hashmap = Hashmap::new(10);
        assert_eq!(hashmap.capacity, 10);
        assert_eq!(hashmap.container.len(), 10);
        for i in 0..10 {
            assert_eq!(hashmap.container[i], 0);
        }
    }

    #[test]
    fn test_add_to_hash() {
        let mut hashmap = Hash::new(10);
        hashmap.add(10, "Test1");
        hashmap.add(11, "Test2");

        assert_eq!(hashmap.container[0], 10);
        assert_eq!(hashmap.dce_id[0], "Test1");
        assert_eq!(hashmap.container[1], 11);
        assert_eq!(hashmap.dce_id[1], "Test2");
    }

    #[test]
    fn test_contains_value() {
        let mut hashmap = Hash::new(10);
        hashmap.container[0] = 10;
        hashmap.container[2] = 12;
        // Collision that had to probe
        hashmap.container[3] = 2;

        assert_eq!(hashmap.contains(10), true);
        assert_eq!(hashmap.contains(12), true);
        assert_eq!(hashmap.contains(2), true);

        assert_eq!(hashmap.contains(1), false);
        assert_eq!(hashmap.contains(11), false);
    }
}
