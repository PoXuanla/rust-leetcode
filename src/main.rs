mod solutions;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        solutions::list_problems();
        return;
    }

    match args[1].parse::<u32>() {
        Ok(problem_id) => solutions::run_problem(problem_id),
        Err(_) => {
            println!("\n❌ 無效的題號：{}", args[1]);
            println!("請輸入有效的數字");
            solutions::list_problems();
        }
    }
}
