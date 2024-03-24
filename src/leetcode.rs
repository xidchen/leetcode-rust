pub struct Leetcode;

impl Leetcode {

    // 1: /problems/two-sum/
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut dic: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
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
        let mut last_seen: std::collections::HashMap<char, i32> = std::collections::HashMap::new();
        let mut start: i32 = 0;
        let mut longest: i32 = 0;
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
        let mut start: i32 = -1;
        let mut ml: i32 = 0;
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

    // 6: /problems/zigzag-conversion/
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 || s.len() < num_rows as usize {
            return s.to_string();
        }
        let mut zigzag: Vec<String> = vec![String::new(); num_rows as usize];
        let mut row: usize = 0;
        let mut step: isize = 1;
        for c in s.chars() {
            zigzag[row].push(c);
            if row == 0 {
                step = 1;
            }
            if row == (num_rows - 1) as usize {
                step = -1;
            }
            row = (row as isize + step) as usize;
        }
        let res: String = zigzag.into_iter().collect();
        res
    }

    // 7: /problems/reverse-integer/
    pub fn reverse(x: i32) -> i32 {
        let mut x: i32 = x;
        let negative: bool = x < 0;
        let mut y: i64 = 0;
        if x < 0 {
            x = match x.checked_neg() {
                Some(negated) => negated,
                None => return 0,
            };
        }
        while x != 0 {
            y = y * 10 + (x % 10) as i64;
            x /= 10;
        }
        if y > i32::MAX as i64 { return 0; }
        let result: i32 = if negative { -y as i32 } else { y as i32 };
        result
    }

    // 8: /problems/string-to-integer-atoi/
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim_start();
        if s.is_empty() { return 0; }
        let mut chars = s.chars();
        let mut sign = 1;
        if let Some(c) = chars.next() {
            if c == '+' || c == '-' {
                sign = if c == '-' { -1 } else { 1 };
            } else if c.is_ascii_digit() {
                chars = s.chars();
            } else { return 0; }
        }
        let mut res: i64 = 0;
        for c in chars {
            if c.is_ascii_digit() {
                res = res * 10 + (c as i64 - '0' as i64);
                if res * sign > i32::MAX as i64 { return i32::MAX; }
                if res * sign < i32::MIN as i64 { return i32::MIN; }
            } else { break; }
        }
        (res * sign) as i32
    }

    // 9: /problems/palindrome-number/
    pub fn is_palindrome(x: i32) -> bool {
        let s = x.to_string();
        s == s.chars().rev().collect::<String>()
    }

    // 10: /problems/regular-expression-matching/
    pub fn is_match(s: String, p: String) -> bool {
        let s_chars: Vec<char> = s.chars().collect();
        let p_chars: Vec<char> = p.chars().collect();
        let mut dp = vec![vec![false; p_chars.len() + 1]; s_chars.len() + 1];
        dp[s_chars.len()][p_chars.len()] = true;
        for i in (0..=s_chars.len()).rev() {
            for j in (0..p_chars.len()).rev() {
                let first_match = i < s_chars.len() &&
                    (p_chars[j] == s_chars[i] || p_chars[j] == '.');
                if j + 1 < p_chars.len() && p_chars[j + 1] == '*' {
                    dp[i][j] = dp[i][j + 2] || (first_match && dp[i + 1][j]);
                } else {
                    dp[i][j] = first_match && dp[i + 1][j + 1];
                }
            }
        }
        dp[0][0]
    }

    // 11: /problems/container-with-most-water/
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut i = 0;
        let mut j = height.len() - 1;
        while i < j {
            max_area = std::cmp::max(
                max_area, std::cmp::min(height[i], height[j]) * (j - i) as i32
            );
            if height[i] < height[j] { i += 1; }
            else { j -= 1; }
        }
        max_area
    }

    // 12: /problems/integer-to-roman/
    pub fn int_to_roman(num: i32) -> String {
        let mapping = vec![
            (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
            (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
            (10, "X"), (9, "IX"), (5, "V"), (4, "IV"),
            (1, "I"),
        ];
        let mut romans = String::new();
        let mut num = num;
        for &(i, roman) in &mapping {
            while i <= num {
                num -= i;
                romans.push_str(roman);
            }
        }
        romans
    }

    // 13: /problems/roman-to-integer/
    pub fn roman_to_int(s: String) -> i32 {
        let mut d: std::collections::HashMap<char, i32> = std::collections::HashMap::new();
        d.insert('I', 1);
        d.insert('V', 5);
        d.insert('X', 10);
        d.insert('L', 50);
        d.insert('C', 100);
        d.insert('D', 500);
        d.insert('M', 1000);
        let mut integer = 0;
        let mut prev_int = 0;
        for roman in s.chars().rev() {
            match d.get(&roman) {
                Some(&value) if value >= prev_int => {
                    prev_int = value;
                    integer += value;
                },
                Some(&value) => {
                    integer -= value;
                },
                _ => (),
            }
        }
        integer
    }

    // 14: /problems/longest-common-prefix/
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() { return String::new(); }
        if strs.len() == 1 { return strs[0].clone(); }
        let mut strs = strs;
        strs.sort();
        let mut result = String::new();
        for (a, b) in strs[0].chars().zip(strs.last().unwrap().chars()) {
            if a == b { result.push(a); }
            else { break; }
        }
        result
    }

}
