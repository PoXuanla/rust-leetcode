# Rust LeetCode 刷題專案

這是一個模組化的 LeetCode 題目管理系統，可以透過命令行輸入題號來執行對應的題目。

## 📊 完成進度

**總題數：5 題**

### 按難度分類
- 🟢 簡單：5 題
- 🟡 中等：0 題
- 🔴 困難：0 題

### 按題型分類
- **陣列 (Array)**: 2 題
- **哈希表 (Hash Table)**: 2 題
- **動態規劃 (Dynamic Programming)**: 1 題
- **鏈表 (Linked List)**: 1 題
- **字符串 (String)**: 1 題

## 專案結構

```
hello-rust/
├── src/
│   ├── main.rs              # 主進入點，處理命令行參數
│   └── solutions/
│       ├── mod.rs           # 題目路由系統
│       ├── p0070.rs         # 70. Climb Stairs
│       └── p0217.rs         # 217. Contains Duplicate
├── Cargo.toml
└── README.md
```

## 設置 Rust 環境

如果你還沒有安裝 Rust，請先執行：

```bash
rustup default stable
```

## 使用方式

### 查看所有可用題目

```bash
cargo run
```

### 執行指定題目

```bash
cargo run 217    # 執行 Contains Duplicate
cargo run 70     # 執行 Climb Stairs
```

### 執行不存在的題目

```bash
cargo run 999    # 顯示：題目 999 尚未實作
```

## 📝 題目列表

### 按題號排序

| # | 題目 | 難度 | 題型 | 狀態 |
|---|------|------|------|------|
| 70 | Climb Stairs | 🟢 簡單 | 動態規劃 | ✅ |
| 121 | Best Time to Buy and Sell Stock | 🟢 簡單 | 陣列, 貪心 | ✅ |
| 206 | Reverse Linked List | 🟢 簡單 | 鏈表 | ✅ |
| 217 | Contains Duplicate | 🟢 簡單 | 陣列, 哈希表 | ✅ |
| 242 | Valid Anagram | 🟢 簡單 | 字符串, 哈希表 | ✅ |

### 按題型分類

**陣列 (Array)**
- [121] Best Time to Buy and Sell Stock 🟢
- [217] Contains Duplicate 🟢

**哈希表 (Hash Table)**
- [217] Contains Duplicate 🟢
- [242] Valid Anagram 🟢

**動態規劃 (Dynamic Programming)**
- [70] Climb Stairs 🟢

**鏈表 (Linked List)**
- [206] Reverse Linked List 🟢

**字符串 (String)**
- [242] Valid Anagram 🟢

## 新增題目

要新增一個題目（例如題號 1）：

1. **建立題目檔案** `src/solutions/p0001.rs`：

```rust
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // 你的實作
}

pub fn solve() {
    println!("\n=== 1. Two Sum ===");
    let result = two_sum(vec![2, 7, 11, 15], 9);
    println!("Output: {:?}", result);
}
```

2. **在 `src/solutions/mod.rs` 中註冊**：

```rust
pub mod p0001;  // 加入這行
pub mod p0070;
pub mod p0217;

pub fn run_problem(problem_id: u32) {
    match problem_id {
        1 => p0001::solve(),     // 加入這行
        70 => p0070::solve(),
        217 => p0217::solve(),
        _ => { /* ... */ }
    }
}
```

3. **更新 `list_problems()` 函數**，加入新題目的資訊。

就這麼簡單！

## 範例輸出

```bash
$ cargo run 217

=== 217. Contains Duplicate ===
Input: [1, 2, 3, 1]
Output: true

Input: [1, 2, 3, 4]
Output: false
```

## 架構設計

系統使用模組化設計：
- 每個題目獨立一個檔案
- 統一的路由系統管理所有題目
- 簡單的命令行介面

這樣的設計讓新增題目變得非常簡單，只需要三個步驟即可！
# rust-leetcode
