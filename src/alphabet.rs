use std::collections::HashMap;

pub fn make_alphabet(nucs: &str) -> HashMap<char, u64> {
    let mut map = HashMap::new();
    for (index, nuc) in nucs.to_uppercase().chars().enumerate() {
        map.insert(nuc, index as u64);
    }
    map
}

#[cfg(test)]
mod test {
    use crate::alphabet::make_alphabet;

    #[test]
    fn test_make_alphabet() {
        let alpha_map = make_alphabet("ABCD");

        assert_eq!(alpha_map.len(), 4);

        assert!(alpha_map.contains_key(&'A'));
        assert_eq!(alpha_map.get(&'A'), Some(&0u64));

        assert!(alpha_map.contains_key(&'B'));
        assert_eq!(alpha_map.get(&'B'), Some(&1u64));

        assert!(alpha_map.contains_key(&'C'));
        assert_eq!(alpha_map.get(&'C'), Some(&2u64));

        assert!(alpha_map.contains_key(&'D'));
        assert_eq!(alpha_map.get(&'D'), Some(&3u64));
    }
}
