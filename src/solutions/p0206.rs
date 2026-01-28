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

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None; //紀錄上一個節點的地址
    
    let mut curr = head;

    while let Some(mut node) = curr {
        
        let next_addr = node.next.take();
        
        node.next = prev;
        
        prev = Some(node);
    
        curr = next_addr;
    }
    
    prev
}

// Helper function to create linked list from vector
fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in vec.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

// Helper function to convert linked list to vector for printing
fn list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = head;
    while let Some(node) = current {
        result.push(node.val);
        current = node.next;
    }
    result
}

pub fn solve() {
    println!("\n=== 206. Reverse Linked List ===");
    
    let test_case1 = vec![1, 2, 3, 4, 5];
    let list1 = vec_to_list(test_case1.clone());
    let result1 = reverse_list(list1);
    println!("Input: {:?}", test_case1);
    println!("Output: {:?}", list_to_vec(result1));
    println!("Expected: [5, 4, 3, 2, 1]");
    
    let test_case2 = vec![1, 2];
    let list2 = vec_to_list(test_case2.clone());
    let result2 = reverse_list(list2);
    println!("\nInput: {:?}", test_case2);
    println!("Output: {:?}", list_to_vec(result2));
    println!("Expected: [2, 1]");
    
    let test_case3: Vec<i32> = vec![];
    let list3 = vec_to_list(test_case3.clone());
    let result3 = reverse_list(list3);
    println!("\nInput: {:?}", test_case3);
    println!("Output: {:?}", list_to_vec(result3));
    println!("Expected: []");
}

