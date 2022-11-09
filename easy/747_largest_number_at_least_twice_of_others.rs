impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut index: i32 = -1;
        let mut second = 0;

        for (i, n) in nums.iter().enumerate() {
            if *n > max {
                second = max;
                max = *n;
                index = i as i32;
            } else if *n > second {
                second = *n;
            }
        }

        if (second * 2 > max) {
            index = -1;
        }

        index
    }
}
