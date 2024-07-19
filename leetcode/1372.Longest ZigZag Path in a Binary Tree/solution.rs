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
    pub fn rec_zig_zag(
        curr_node: &Option<Rc<RefCell<TreeNode>>>,
        is_left_node: bool,
        streak: i32,
        longest_streak: &mut i32,
    ) {
        if curr_node.is_none() {
            return;
        }
        *longest_streak = (*longest_streak).max(streak);
        let curr = curr_node.as_ref().unwrap().borrow();
        if curr.left.is_some() {
            // If originally we came from the left (meaning current node is the left_node of the parent), we reset the streak when going to the left
            // On the other hand, we continue the streak if we go to the right, as we are making a ZigZag
            if is_left_node {
                Self::rec_zig_zag(&curr.left, true, 1, longest_streak);
            } else {
                Self::rec_zig_zag(&curr.left, true, streak + 1, longest_streak);
            }
        }

        if curr.right.is_some() {
            // Same logic here, if we were a right node, then if we go to the right again we reset the streak
            if !is_left_node {
                Self::rec_zig_zag(&curr.right, false, 1, longest_streak);
            } else {
                Self::rec_zig_zag(&curr.right, false, streak + 1, longest_streak);
            }
        }
    }

    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut longest_streak = i32::MIN;
        Self::rec_zig_zag(&root, false, 0, &mut longest_streak);
        longest_streak
    }
}
