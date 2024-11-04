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
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>, 
        mut l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut current_node = &mut dummy_head;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let current_sum = match (l1.take(), l2.take()) {
                (Some(node1), Some(node2)) => {
                    l1 = node1.next;
                    l2 = node2.next;
                    node1.val + node2.val + carry
                },
                (Some(node1), None) => {
                    l1 = node1.next;
                    node1.val + carry
                },
                (None, Some(node2)) => {
                    l2 = node2.next;
                    node2.val + carry
                },
                (None, None) => carry,
            };

            carry = current_sum / 10;
            let new_digit = current_sum % 10;

            let new_node = Box::new(ListNode::new(new_digit));
            current_node.next = Some(new_node);
            current_node = current_node.next.as_mut().unwrap();
        }

        dummy_head.next
    }
}