pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left=  0;
    let mut right = numbers.len()-1;

    while left < right {
        let sum = numbers[left] + numbers[right];

        if sum == target{
            return vec![(left as i32)+1,(right as i32)+1];
        }else if sum > target{
            right = right - 1;
        }else{
            left = left +1;
        }
    }
    vec![]
}

pub fn solve() {
    println!("\n=== 167. Two Sum II - Input Array Is Sorted ===");
    
    let test_case1 = vec![2, 7, 11, 15];
    let target1 = 9;
    let result1 = two_sum(test_case1.clone(), target1);
    println!("Input: numbers = {:?}, target = {}", test_case1, target1);
    println!("Output: {:?}", result1);
    println!("Expected: [1, 2]");
    
    let test_case2 = vec![2, 3, 4];
    let target2 = 6;
    let result2 = two_sum(test_case2.clone(), target2);
    println!("\nInput: numbers = {:?}, target = {}", test_case2, target2);
    println!("Output: {:?}", result2);
    println!("Expected: [1, 3]");
    
    let test_case3 = vec![-1, 0];
    let target3 = -1;
    let result3 = two_sum(test_case3.clone(), target3);
    println!("\nInput: numbers = {:?}, target = {}", test_case3, target3);
    println!("Output: {:?}", result3);
    println!("Expected: [1, 2]");
}

