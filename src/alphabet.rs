use std::collections::HashMap;

pub fn make_alphabet(nucs: &str) -> HashMap<char, u64> {
    let mut map = HashMap::new();
    for (index, nuc) in nucs.to_uppercase().chars().enumerate() {
        map.insert(nuc, index as u64);
    }
    map
}
