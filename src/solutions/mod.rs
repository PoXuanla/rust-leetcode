pub mod p0070;
pub mod p0121;
pub mod p0217;
pub mod p0242;

pub fn run_problem(problem_id: u32) {
    match problem_id {
        70 => p0070::solve(),
        121 => p0121::solve(),
        217 => p0217::solve(),
        242 => p0242::solve(),
        _ => {
            println!("\n❌ 題目 {} 尚未實作", problem_id);
            println!("\n目前可用的題目：");
            println!("  - 70:  Climb Stairs");
            println!("  - 121: Best Time to Buy and Sell Stock");
            println!("  - 217: Contains Duplicate");
            println!("  - 242: Valid Anagram");
            println!("\n使用方式: cargo run <題號>");
        }
    }
}

pub fn list_problems() {
    println!("\n=== 可用的 LeetCode 題目 ===\n");
    println!("  70  - Climb Stairs");
    println!("  121 - Best Time to Buy and Sell Stock");
    println!("  217 - Contains Duplicate");
    println!("  242 - Valid Anagram");
    println!("\n使用方式: cargo run <題號>");
    println!("範例: cargo run 217\n");
}
