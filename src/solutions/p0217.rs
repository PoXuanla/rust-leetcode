use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::new();

    for num in nums {
        if seen.contains(&num) {
            return true;
        }
        seen.insert(num);
    }
    false
}

pub fn solve() {
    println!("\n=== 217. Contains Duplicate ===");
    
    let test_case1 = vec![1, 2, 3, 1];
    let result1 = contains_duplicate(test_case1.clone());
    println!("Input: {:?}", test_case1);
    println!("Output: {}", result1);
    
    let test_case2 = vec![1, 2, 3, 4];
    let result2 = contains_duplicate(test_case2.clone());
    println!("\nInput: {:?}", test_case2);
    println!("Output: {}", result2);
}
