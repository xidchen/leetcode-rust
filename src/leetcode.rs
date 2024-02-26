use std::collections::HashMap;

pub struct Leetcode;

impl Leetcode {

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
        let mut start = 0;
        let mut longest = 0;
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

    // 5: /problems/longest-palindromic-substring/
    pub fn longest_palindromic_substring(s: String) -> String {
        if s.len() < 2 || s.chars().eq(s.chars().rev()) {
            return s.to_string();
        }
        let mut start = -1;
        let mut ml = 0;
        for i in 0..s.len() {
            let odd = if i as i32 - ml - 1 >= 0 {
                &s[i - ml as usize - 1..=i]
            } else {""};
            let even = if i as i32 - ml >= 0 {
                &s[i - ml as usize..=i]
            } else {""};
            if !odd.is_empty() && odd.chars().eq(odd.chars().rev()) {
                start = i as i32 - ml - 1;
                ml += 2;
                continue;
            }
            if !even.is_empty() && even.chars().eq(even.chars().rev()) {
                start = i as i32 - ml;
                ml += 1;
            }
        }
        s.chars().skip(start as usize).take(ml as usize).collect()
    }

}
