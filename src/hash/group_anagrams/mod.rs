struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        for word in &strs {
            let mut bytes = word.clone().into_bytes();
            bytes.sort();
            let keys = String::from_utf8(bytes).unwrap();
            let values = map.entry(keys).or_insert(Vec::new());
            values.push(word.clone())
        }
        map.into_values().collect()
    }
}
