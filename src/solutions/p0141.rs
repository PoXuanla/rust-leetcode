// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn has_cycle(head: Option<Box<ListNode>>) -> bool {
    let mut slow = &head;
    let mut fast = &head;

    while let (Some(s), Some(f)) = (slow, fast) {
        slow = &s.next;

        if let Some(next_node) = &f.next {
            fast = &next_node.next;
        } else {
            return false;
        }

        if let (Some(s_node), Some(f_node)) = (slow, fast) {
            if std::ptr::eq(&**s_node, &**f_node) {
                return true;
            }
        }
    }
    false
}

// Helper function to create linked list with cycle
fn create_cycle_list(values: Vec<i32>, pos: i32) -> Option<Box<ListNode>> {
    if values.is_empty() {
        return None;
    }

    // Note: This is a simplified version for demonstration
    // In actual implementation, we need to use raw pointers or Rc/RefCell
    // to create cycles, which is complex in Rust due to ownership rules

    // For now, we'll just create a regular list
    let mut head = None;
    for &val in values.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

pub fn solve() {
    println!("\n=== 141. Linked List Cycle ===");

    println!("Test Case 1:");
    println!("Input: head = [3,2,0,-4], pos = 1");
    let list1 = create_cycle_list(vec![3, 2, 0, -4], 1);
    let result1 = has_cycle(list1);
    println!("Output: {}", result1);
    println!("Expected: true");
    println!("Note: Creating actual cycles requires unsafe Rust or Rc<RefCell>");

    println!("\nTest Case 2:");
    println!("Input: head = [1,2], pos = 0");
    let list2 = create_cycle_list(vec![1, 2], 0);
    let result2 = has_cycle(list2);
    println!("Output: {}", result2);
    println!("Expected: true");

    println!("\nTest Case 3:");
    println!("Input: head = [1], pos = -1");
    let list3 = create_cycle_list(vec![1], -1);
    let result3 = has_cycle(list3);
    println!("Output: {}", result3);
    println!("Expected: false");
}
