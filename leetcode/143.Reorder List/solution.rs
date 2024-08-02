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
    pub fn reorder_list(mut head: &mut Option<Box<ListNode>>) {
        let mut stack: Vec<Option<Box<ListNode>>> = Vec::new();
        let mut current_node = head.take();
        while let Some(mut node) = current_node {
            let mut next_node = node.next.take();
            stack.push(Some(node));
            current_node = next_node;
        }

        let mut list_length = stack.len();
        let mut left = 0;
        let mut right = list_length - 1;

        while left < right {
            let mut left_node = stack[left].take();
            left_node.as_mut().unwrap().next = stack[right].take();
            stack[left] = left_node.take();
            left += 1;
            right -= 1;
        }
        for i in (0..left).rev() {
            if stack[i].is_some() {
                stack[i].as_mut().unwrap().next.as_mut().unwrap().next = stack[i + 1].take()
            }
        }
        *head = stack[0].take();
    }
}
