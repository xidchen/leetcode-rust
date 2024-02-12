use std::collections::HashMap;

// 1: /problems/two-sum/
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut dic: HashMap<i32, usize> = HashMap::new();
    for (i, &n) in nums.iter().enumerate() {
        if let Some(&prev_index) = dic.get(&n) {
            return vec![prev_index as i32, i as i32];
        }
        dic.insert(target - n, i);
    }
    Vec::new()
}

// 3: /problems/longest-substring-without-repeating-characters/
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut last_seen: HashMap<char, i32> = HashMap::new();
    let mut start:i32 = 0;
    let mut longest:i32 = 0;
    for (i, c) in s.chars().enumerate() {
        if let Some(&last_index) = last_seen.get(&c) {
            if last_index >= start {
                start = last_index + 1;
            }
        }
        longest = longest.max(i as i32 - start + 1);
        last_seen.insert(c, i as i32);
    }
    longest
}
