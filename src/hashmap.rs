/// A very simple hash set implementation that uses
/// linear probing to handle collisions. This is intended
/// to be extremely lightweight to improve performance.
#[derive(Clone)]
pub struct Hashmap {
    pub container: Vec<u64>,
    pub(crate) dce_id: Vec<Vec<String>>,
    capacity: u64,
    pub index: usize,
}

impl Hashmap {
    pub fn new(capacity: usize) -> Hashmap {
        Hashmap {
            container: vec![0; capacity],
            dce_id: vec![vec![0.to_string(); 1]; capacity],
            capacity: capacity as u64,
            index: 0,
        }
    }

    // Add the given value to the set.
    pub fn add(&mut self, value: u64, id: String) {
        let hv = value % self.capacity;

        // We may now cast hv to a usize because we're sure
        // that it is < self.size and will therefore fit.
        let hv_index = hv as usize;
        let mut probed_index = hv_index;

        //if this is a duplicate that already has an existing ID inserted,
        //we don't want to probe past it due to the index already being "occupied"
        if self.container[probed_index] == value {
            self.dce_id[probed_index].push(id);
            self.index = probed_index;
        }

        //If the sequence value isn't in container at the computed index, it is either:
        // a) not inserted yet or b) had a collision and is in another index
        else{
            while self.container[probed_index] != 0 { //start probing as normal for a new insertion
                probed_index += 1;
                //But check here to see if the move puts us at the right seq value in container,
                //even though we haven't yet encountered an empty index
                if self.container[probed_index] == value {
                    //push it onto the id vector because it can't be a new insertion now
                    self.dce_id[probed_index].push(id.clone());
                    self.container[probed_index] = value;
                    self.index = probed_index;
                    break;
                }

                //If we get here, it is a new insertion. Proceed as "normal".
                if probed_index >= self.capacity as usize {
                    probed_index = 0;
                }

                if probed_index == hv_index {
                    panic!("hash full");
                }
            }
            self.container[probed_index] = value;
            self.dce_id[probed_index][0] = id.clone();
            self.index = probed_index;
        }

        /*self.container[probed_index] = value;
        if self.dce_id[probed_index][0] != 0.to_string() {
            self.dce_id[probed_index].push(id);
        }
        else{
            self.dce_id[probed_index][0] = id;
        }*/
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
