use std::i32;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    // TODO: 實作你的解答
    let mut good_profit  = 0;
    let mut min_purchase_price = i32::MAX;
     
    for price in prices {
        good_profit = good_profit.max(price - min_purchase_price);
        min_purchase_price = min_purchase_price.min(price);
    }
    return good_profit;
}

pub fn solve() {
    println!("\n=== 121. Best Time to Buy and Sell Stock ===");
    
    let test_case1 = vec![7, 1, 5, 3, 6, 4];
    let result1 = max_profit(test_case1.clone());
    println!("Input: {:?}", test_case1);
    println!("Output: {}", result1);
    println!("Expected: 5");
    
    let test_case2 = vec![7, 6, 4, 3, 1];
    let result2 = max_profit(test_case2.clone());
    println!("\nInput: {:?}", test_case2);
    println!("Output: {}", result2);
    println!("Expected: 0");
    
    let test_case3 = vec![2, 4, 1];
    let result3 = max_profit(test_case3.clone());
    println!("\nInput: {:?}", test_case3);
    println!("Output: {}", result3);
    println!("Expected: 2");
}

