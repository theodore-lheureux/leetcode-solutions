use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut result = Vec::new();

        for n in nums {
            *map.entry(n).or_insert(1) += 1;
        }

        let mut sorted_map: Vec<(&i32, &i32)> = map.iter().collect();
        sorted_map.sort_unstable_by(|a, b| a.1.cmp(b.1));

        for i in sorted_map.len() - k as usize..sorted_map.len() {
            result.push(*sorted_map[i as usize].0);
        }

        result
    }
}
