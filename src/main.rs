use std::cell::RefCell;
use std::rc::Rc;
use crate::structure::{ListNode, TreeNode};

mod structure;

fn main() {
    println!("Hello, world!");
}


struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<crate::structure::ListNode>>) -> Option<Box<crate::structure::ListNode>> {
        let mut prev: Option<Box<crate::structure::ListNode>> = None;
        let mut curr: Option<Box<crate::structure::ListNode>> = head;
        while let Some(mut node) = curr {
            let next = node.next;
            node.next = prev;
            prev = Some(node);
            curr = next;
        }
        curr
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            return Self::check(Some(node.clone()), Some(node.clone()))
        }
        false
    }

    pub fn check(l: Option<Rc<RefCell<TreeNode>>>, r: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (l, r) {
            (Some(l_node), Some(r_node)) => {
                let x = l_node.borrow();
                let y = r_node.borrow();
                x.val == y.val && Self::check(x.left.clone(), y.right.clone()) && Self::check(x.right.clone(), y.left.clone())
            },
            (None, None) => true,
            _ => false
        }
    }

    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let i: i32 = -1;
        loop {
            for str in strs {

            }
        }
        if i == -1 {
            return String::new();
        }
        strs[0][0..i]
    }

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut dummy = ListNode::new(0);
        let mut curr = &mut dummy;

        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;
            if let Some(l1_node) = l1 {
                sum += l1_node.val;
                l1 = l1_node.next;
            }
            if let Some(l2_node) = l2 {
                sum += l2_node.val;
                l2 = l2_node.next;
            }
            carry = sum / 10;
            let n = ListNode::new(sum % 10);
            curr.next = Some(Box::new(n));
            curr = curr.next.as_mut().unwrap();
        }
        dummy.next
    }
}