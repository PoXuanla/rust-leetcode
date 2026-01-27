pub fn climb_stairs(n: i32) -> i32 {
    if n < 2 {
        return n;
    }

    let mut dp = vec![0; (n + 1) as usize];
    dp[1] = 1;
    dp[2] = 2;

    for i in 3..=n as usize {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    return dp[n as usize];
}

pub fn solve() {
    println!("\n=== 70. Climb Stairs ===");
    
    let test_case1 = 2;
    let result1 = climb_stairs(test_case1);
    println!("Input: n = {}", test_case1);
    println!("Output: {}", result1);
    
    let test_case2 = 3;
    let result2 = climb_stairs(test_case2);
    println!("\nInput: n = {}", test_case2);
    println!("Output: {}", result2);
    
    let test_case3 = 5;
    let result3 = climb_stairs(test_case3);
    println!("\nInput: n = {}", test_case3);
    println!("Output: {}", result3);
}
