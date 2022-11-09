// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    fn find_sum(num1: i32, num2: i32, remainder: &mut bool) -> i32 {
        let mut sum = num1 + num2;

        if *remainder {
            sum += 1;
        }

        let sum = if sum > 9 {
            *remainder = true;
            sum - 10
        } else {
            *remainder = false;
            sum
        };

        sum
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut num1 = 0;
        let mut num2 = 0;

        let mut node_l1 = &l1.as_ref().unwrap().next;
        let mut node_l2 = &l2.as_ref().unwrap().next;
        let mut remainder = false;

        let mut result = Some(Box::new(ListNode::new(Solution::find_sum(
            l1.as_ref().unwrap().val,
            l2.as_ref().unwrap().val,
            &mut remainder,
        ))));

        let mut node_result = &mut result;

        loop {
            match node_l1 {
                None => {
                    num1 = 0;
                }
                Some(x) => {
                    num1 = x.val;
                    node_l1 = &x.as_ref().next;
                }
            }
            match node_l2 {
                None => {
                    num2 = 0;
                }
                Some(x) => {
                    num2 = x.val;
                    node_l2 = &x.as_ref().next;
                }
            }
            let sum = Solution::find_sum(num1, num2, &mut remainder);

            if sum == 0
                && remainder == false
                && *node_l1 == None
                && *node_l2 == None
            {
                break;
            };

            node_result.as_mut().unwrap().next =
                Some(Box::new(ListNode::new(sum)));
            node_result = &mut node_result.as_mut().unwrap().next;
        }
        result
    }
}
