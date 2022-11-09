// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        let mut all_none = false;
        let mut matrix = Vec::<Vec<String>>::new();

        matrix.push(Vec::new());
        matrix[0]
            .push(root.as_ref().unwrap().as_ref().borrow().val.to_string());

        let mut current_row: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![root];

        while !all_none {
            all_none = true;
            current_row = next_row(current_row, &mut all_none);

            if !all_none {
                add_row(&mut matrix);
                matrix.push(row_string_vec(&current_row));
            }
        }
        matrix
    }
}

pub fn row_string_vec(
    current_row: &Vec<Option<Rc<RefCell<TreeNode>>>>,
) -> Vec<String> {
    let mut row = Vec::new();

    for i in current_row {
        match i {
            Some(x) => row.push(x.as_ref().borrow().val.to_string()),
            None => row.push("".to_owned()),
        }
        row.push("".to_owned());
    }

    row.pop();
    row
}

pub fn next_row(
    current_row: Vec<Option<Rc<RefCell<TreeNode>>>>,
    all_none: &mut bool,
) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let mut next_row = Vec::new();

    for node_option in current_row {
        if let Some(node) = node_option {
            let node = Rc::try_unwrap(node).unwrap().into_inner();
            let left = node.left;
            let right = node.right;

            if &left != &None || &right != &None {
                *all_none = false;
            }

            next_row.push(left);
            next_row.push(right);
        } else {
            next_row.push(None);
            next_row.push(None);
        }
    }

    next_row
}

pub fn add_row(matrix: &mut Vec<Vec<String>>) -> () {
    let len = AsRef::<Vec<Vec<String>>>::as_ref(matrix).len();

    for row in matrix.iter_mut() {
        let mut temp = Vec::new();

        for i in [..=len] {
            temp.push("".to_owned());
        }

        for col in row.into_iter() {
            temp.push(col.clone());
            temp.push("".to_owned());
        }

        temp.pop();

        for i in [..=len] {
            temp.push("".to_owned());
        }

        *row = temp;
    }
}
