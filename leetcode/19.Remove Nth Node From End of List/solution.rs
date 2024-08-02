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
    pub fn get_list_length(curr: &Option<Box<ListNode>>, count: usize) -> usize {
        if curr.is_none() {
            return count;
        }
        Self::get_list_length(&curr.as_ref().unwrap().next, count + 1)
    }
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let list_length = Self::get_list_length(&head, 0);
        let target_node_index = list_length - n as usize + 1;
        if target_node_index == 1 {
            return head.as_mut().unwrap().next.take();
        }

        let mut move_node = head.as_mut();
        let mut curr_index = 1;
        while let Some(curr) = move_node {
            if curr_index == target_node_index - 1 {
                curr.next = curr.next.as_mut().unwrap().next.take();
                break;
            }
            move_node = curr.next.as_mut();
            curr_index += 1;
        }
        head
    }
}
