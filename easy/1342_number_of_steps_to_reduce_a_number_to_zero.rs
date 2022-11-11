impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut n_steps = 0;
        let mut num = num;

        while num != 0 {
            if num % 2 == 1 {
                num -= 1;
            } else {
                num /= 2;
            }
            n_steps += 1;
        }

        n_steps
    }
}
