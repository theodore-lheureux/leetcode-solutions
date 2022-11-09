impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums = nums1.clone();

        for i in nums2 {
            nums.push(i);
        }

        nums.sort();

        // return value
        if nums.len() == 1 {
            return nums[0] as f64;
        }

        if nums.len() % 2 == 1 {
            return nums[nums.len() / 2] as f64;
        }

        let middleI = nums.len() / 2;

        ((nums[middleI] + nums[middleI - 1]) as f64) / 2f64
    }
}
