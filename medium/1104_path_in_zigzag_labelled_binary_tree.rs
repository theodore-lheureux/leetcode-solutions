fn get_parent_node_label(pos: f32) -> i32 {
    let result = (2f32).powf(pos - 1f32);
    // This is a wacky af way to avoid floating point errors.
    // Might not work in all cases, might actually be garbage.
    let r = (result - 0.3).round();
    r as i32
}

fn get_node_pos(label: i32) -> f32 {
    (label as f32).log2()
}

fn normalize_label(label: i32) -> i32 {
    let row = (get_node_pos(label) + 1f32) as u32;

    if row % 2 == 1 {
        label
    } else {
        let lower_bound = 2i32.pow(row - 1);
        let higher_bound = 2i32.pow(row) - 1;

        higher_bound - label + lower_bound
    }
}

impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        let mut output: Vec<i32> = vec![label];
        let mut current_label = normalize_label(label);

        while current_label > 1 {
            let node_pos = get_node_pos(current_label);
            let parent_node_label = get_parent_node_label(node_pos);
            let inversed_parent = normalize_label(parent_node_label);

            output.insert(0, inversed_parent);

            current_label = parent_node_label;
        }

        output
    }
}
