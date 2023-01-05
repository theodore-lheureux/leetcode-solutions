impl Solution {
    pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
        
        let (mut maxId, mut max) = (logs[0][0], logs[0][1]);
        let mut previousLog = &logs[0];
        
        for (i, log) in logs[1..].iter().enumerate() {
            let n = log[1] - previousLog[1];
            if n >= max {
                if n == max {
                    if log[0] < maxId {
                        maxId = log[0];
                    }
                } else {
                    maxId = log[0];
                }
                max = n;
            }
            previousLog = log;
        }

        maxId

    }
}
