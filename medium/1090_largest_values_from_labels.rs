use std::collections::HashMap;

impl Solution {
    pub fn largest_vals_from_labels(
        values: Vec<i32>,
        labels: Vec<i32>,
        num_wanted: i32,
        use_limit: i32,
    ) -> i32 {
        let mut sorted_map: Vec<(i32, i32)> = Vec::new();

        for i in 0..values.len() {
            sorted_map.push((values[i], labels[i]));
        }

        sorted_map.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        let mut i = 0_usize;
        let mut subset = 0;
        let mut subset_len = 0;
        let mut counts = HashMap::new();

        while subset_len < num_wanted && i < sorted_map.len() {
            *counts.entry(sorted_map[i].1).or_insert(0) += 1;

            if counts.get(&sorted_map[i].1).unwrap() <= &use_limit {
                subset += sorted_map[i].0;
                subset_len += 1;
            }

            i += 1;
        }

        subset
    }
}
