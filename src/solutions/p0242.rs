pub fn is_anagram(s: String, t: String) -> bool {
    // TODO: 實作你的解答ㄌ
    let mut s1: Vec<char> = s.chars().collect();
    let mut t1: Vec<char> = t.chars().collect();

    s1.sort();
    t1.sort();

    if s1 == t1 {
      return true
    }
    false
    
}

pub fn solve() {
    println!("\n=== 242. Valid Anagram ===");
    
    let test_case1_s = "anagram".to_string();
    let test_case1_t = "nagaram".to_string();
    let result1 = is_anagram(test_case1_s.clone(), test_case1_t.clone());
    println!("Input: s = \"{}\", t = \"{}\"", test_case1_s, test_case1_t);
    println!("Output: {}", result1);
    
    let test_case2_s = "rat".to_string();
    let test_case2_t = "car".to_string();
    let result2 = is_anagram(test_case2_s.clone(), test_case2_t.clone());
    println!("\nInput: s = \"{}\", t = \"{}\"", test_case2_s, test_case2_t);
    println!("Output: {}", result2);
    
    let test_case3_s = "a".to_string();
    let test_case3_t = "ab".to_string();
    let result3 = is_anagram(test_case3_s.clone(), test_case3_t.clone());
    println!("\nInput: s = \"{}\", t = \"{}\"", test_case3_s, test_case3_t);
    println!("Output: {}", result3);
}

