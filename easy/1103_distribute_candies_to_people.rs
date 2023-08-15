impl Solution {
    pub fn distribute_candies(mut candies: i32, num_people: i32) -> Vec<i32> {
        let mut distribution: Vec<i32> = vec![0; num_people as usize];
        let mut n = 1;

        while candies > 0 {
            for people in distribution.iter_mut() {
                if n <= candies {
                    *people += n;
                    candies -= n;
                } else {
                    *people += candies;
                    candies = 0;
                }
                n += 1;
            }
        }
        distribution
    }
}
