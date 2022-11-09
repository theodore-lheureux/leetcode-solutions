impl Solution {
    // Could be further optimized by checking against a list of known prime numbers
    // and their multiples for the dividors.
    pub fn check_perfect_number(num: i32) -> bool {
        
        let mut result: i32 = 1;
        let max = ((num as f32).sqrt() as i32) + 1;

        if num == 1 {
            return false;
        }

        if num % 2 == 0 {
            result += 2 + num / 2;

            let mut i: i32 = 4;

            while i < max {
                if num % i == 0 {
                    result += i + num / i;
                }
                i += 2;
            }

        }

        let mut i: i32 = 3;

        while i < max {
            if num % i == 0 {
                result += i + num / i;
            }
            i += 2;
        }
        result == num

    }

}
