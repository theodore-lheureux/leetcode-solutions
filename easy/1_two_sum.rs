use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm: HashMap<i32, i32> = HashMap::new();

        for (i, value) in nums.iter().enumerate() {
            match hm.get(&(target - value)) {
                None => (),
                Some(&j) => return Vec::from([i as i32, j]),
            }
            hm.insert(*value, i as i32);
        }

        vec![0, 3]
    }
}
