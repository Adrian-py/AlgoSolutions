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
    pub fn zigzag_level_order(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return Vec::new();
        }

        let mut traversal_stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![root.take()];
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut row_num = 0;

        while !traversal_stack.is_empty() {
            let mut curr_row = Vec::new();
            let mut next_row: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();

            if row_num % 2 == 1 {
                traversal_stack.reverse();
            }

            for curr in traversal_stack.iter() {
                let mut curr_node = (*curr).as_ref().unwrap();
                let mut curr_borrow = curr_node.borrow_mut();

                curr_row.push(curr_borrow.val);
                if row_num % 2 == 1 {
                    if curr_borrow.right.is_some() {
                        next_row.push(curr_borrow.right.take());
                    }
                    if curr_borrow.left.is_some() {
                        next_row.push(curr_borrow.left.take());
                    }
                    continue;
                }

                if curr_borrow.left.is_some() {
                    next_row.push(curr_borrow.left.take());
                }

                if curr_borrow.right.is_some() {
                    next_row.push(curr_borrow.right.take());
                }
            }

            if row_num % 2 == 1 {
                next_row.reverse();
            }

            ans.push(curr_row);
            traversal_stack = next_row.to_vec();
            row_num += 1;
        }

        ans
    }
}
