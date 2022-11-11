use std::collections::HashMap;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result: HashMap<i32, i32> = HashMap::new();

        if nums1.len() >= nums2.len() {
            calculate(&mut result, nums1, nums2);
        } else {
            calculate(&mut result, nums2, nums1);
        }

        result.into_keys().collect()
    }
}

pub fn calculate(
    result: &mut HashMap<i32, i32>,
    nums1: Vec<i32>,
    nums2: Vec<i32>,
) {
    for i in nums1 {
        if nums2.contains(&i) {
            result.insert(i, 0);
        }
    }
}
