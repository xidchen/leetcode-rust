use crate::structs::ListNode;

struct ListNodeWrapper(Box<ListNode>);

impl Ord for ListNodeWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.val.cmp(&self.0.val)
    }
}

impl PartialOrd for ListNodeWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ListNodeWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.val == other.0.val
    }
}

impl Eq for ListNodeWrapper {}

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

    // 2: /problems/add-two-numbers/
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut l1: Option<Box<ListNode>> = l1;
        let mut l2: Option<Box<ListNode>> = l2;
        let mut dummy: Box<ListNode> = Box::new(ListNode::new(0));
        let mut current: &mut Box<ListNode> = &mut dummy;
        let mut carry: i32 = 0;
        while l1.is_some() || l2.is_some() || carry > 0 {
            if let Some(l1_node) = &l1 {
                carry += l1_node.val;
                l1 = l1_node.next.clone();
            }
            if let Some(l2_node) = &l2 {
                carry += l2_node.val;
                l2 = l2_node.next.clone();
            }
            current.next = Some(Box::new(ListNode::new(carry % 10)));
            current = current.next.as_mut()?;
            carry /= 10;
        }
        dummy.next
    }

    // 3: /problems/longest-substring-without-repeating-characters/
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_seen: std::collections::HashMap<char, i32> = std::collections::HashMap::new();
        let mut start: i32 = 0;
        let mut longest: i32 = 0;
        for (i, c) in s.chars().enumerate() {
            if let Some(&last_index) = last_seen.get(&c) {
                if last_index >= start { start = last_index + 1; }
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
            } else { "" };
            let even = if i as i32 - ml >= 0 {
                &s[i - ml as usize..=i]
            } else { "" };
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
            if height[i] < height[j] { i += 1; } else { j -= 1; }
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
            if a == b { result.push(a); } else { break; }
        }
        result
    }

    // 15: /problems/3sum
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut dic: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
        let mut res: Vec<Vec<i32>> = Vec::new();
        for &n in &nums { *dic.entry(n).or_insert(0) += 1; }
        let mut sorted_nums: Vec<i32> = dic.keys().cloned().collect();
        sorted_nums.sort_unstable();
        for (i, &x) in sorted_nums.iter().enumerate() {
            if x == 0 {
                if *dic.get(&x).unwrap_or(&0) > 2 { res.push(vec![0, 0, 0]); }
            } else if *dic.get(&x).unwrap_or(&0) > 1 && dic.contains_key(&(-2 * x)) {
                res.push(vec![x, x, -2 * x]);
            }
            if x < 0 {
                for j in (i + 1)..sorted_nums.len() {
                    let y = sorted_nums[j];
                    let z = -x - y;
                    if z <= y { break; }
                    if dic.contains_key(&z) && z != y { res.push(vec![x, y, z]); }
                }
            }
        }
        res
    }

    // 16: /problems/3sum-closest/
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut nums = nums;
        nums.sort();
        let mut res = nums[0] + nums[1] + nums[2];
        for i in 0..n - 2 {
            let (mut j, mut k) = (i + 1, n - 1);
            if nums[i] + nums[j] + nums[j + 1] >= target { k = j + 1; }
            if nums[i] + nums[k - 1] + nums[k] <= target { j = k - 1; }
            while j < k {
                let s = nums[i] + nums[j] + nums[k];
                if (target - s).abs() < (target - res).abs() { res = s; }
                if s == target { return res; }
                if s < target { j += 1; }
                if s > target { k -= 1; }
            }
        }
        res
    }

    // 17: /problems/letter-combinations-of-a-phone-number/
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();
        if digits.is_empty() || digits.contains('0') || digits.contains('1') {
            return results;
        }
        let mapping: std::collections::HashMap<char, Vec<char>> = [
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z']),
        ].iter().cloned().collect();
        results.push(String::new());
        for digit in digits.chars() {
            let mut temp: Vec<String> = Vec::new();
            for result in &results {
                for letter in mapping[&digit].iter() {
                    temp.push(format!("{}{}", result, letter));
                }
            }
            results = temp;
        }
        results
    }

    // 18: /problems/4sum/
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn k_sum(n: Vec<i64>, t: i64, k: i64) -> Vec<Vec<i32>> {
            let mut res: Vec<Vec<i32>> = Vec::new();
            if n.len() < k as usize || t < n[0] * k || n[n.len() - 1] * k < t { return res; }
            if k == 2 { return two_sum(n, t); }
            for i in 0..n.len() {
                if i == 0 || n[i - 1] != n[i] {
                    for mut set in k_sum(n[i + 1..].to_vec(), t - n[i], k - 1) {
                        set.insert(0, n[i] as i32);
                        res.push(set);
                    }
                }
            }
            res
        }

        fn two_sum(n: Vec<i64>, t: i64) -> Vec<Vec<i32>> {
            let mut res: Vec<Vec<i32>> = Vec::new();
            let (mut lo, mut hi) = (0, n.len() - 1);
            while lo < hi {
                let sum: i64 = n[lo] + n[hi];
                if sum < t || (0 < lo && n[lo] == n[lo - 1]) { lo += 1; }
                else if t < sum || (hi < n.len() - 1 && n[hi] == n[hi + 1]) { hi -= 1; }
                else {
                    res.push(vec![n[lo] as i32, n[hi] as i32]);
                    lo += 1;
                    hi -= 1;
                }
            }
            res
        }

        let mut nums: Vec<i64> = nums.clone().into_iter().map(|x| x as i64).collect();
        nums.sort();
        k_sum(nums, target as i64, 4)
    }

    // 19: /problems/remove-nth-node-from-end-of-list/
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut first = dummy.clone();
        let mut second = &mut dummy;
        for _ in 0..n {
            if let Some(node) = first.next.take() {
                first = node;
            } else {
                return dummy.next;
            }
        }
        while first.next.is_some() {
            first = first.next.unwrap();
            second = second.next.as_mut().unwrap();
        }
        let next = second.next.take();
        second.next = next.and_then(|node| node.next);
        dummy.next
    }

    // 20: /problems/valid-parentheses/
    pub fn is_valid(s: String) -> bool {
        let d: std::collections::HashMap<char, char> = [
            ('(', ')'), ('[', ']'), ('{', '}')
        ].iter().cloned().collect();
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            if d.contains_key(&c) { stack.push(c); }
            else { if stack.is_empty() || d[&stack.pop().unwrap()] != c { return false; } }
        }
        stack.is_empty()
    }

    // 21: /problems/merge-two-sorted-lists/
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy: Box<ListNode> = Box::new(ListNode::new(0));
        let mut prev: &mut Box<ListNode> = &mut dummy;
        while let (
            Some(node1),
            Some(node2)
        ) = (l1.as_mut(), l2.as_mut()) {
            if node1.val < node2.val {
                let next: Option<Box<ListNode>> = node1.next.take();
                prev.next = l1;
                l1 = next;
            } else {
                let next: Option<Box<ListNode>> = node2.next.take();
                prev.next = l2;
                l2 = next;
            }
            prev = prev.next.as_mut()?;
        }
        prev.next = if l1.is_some() { l1 } else { l2 };
        dummy.next
    }

    // 22: /problems/generate-parentheses/
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn backtrack(res: &mut Vec<String>, s: String, left: i32, right: i32, n: i32) {
            if s.len() == (2 * n) as usize {
                res.push(s);
                return;
            }
            if left < n { backtrack(res, s.clone() + "(", left + 1, right, n); }
            if right < left { backtrack(res, s.clone() + ")", left, right + 1, n); }
        }

        let mut result = Vec::new();
        backtrack(&mut result, String::new(), 0, 0, n);
        result
    }

    // 23: /problems/merge-k-sorted-lists/
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = std::collections::BinaryHeap::new();
        for list in lists.into_iter().flatten() { heap.push(ListNodeWrapper(list)); }
        let mut dummy = ListNode::new(0);
        let mut prev = &mut dummy;
        while let Some(ListNodeWrapper(mut node)) = heap.pop() {
            if let Some(next) = node.next.take() { heap.push(ListNodeWrapper(next)); }
            prev.next = Some(node);
            prev = prev.next.as_mut()?;
        }
        dummy.next
    }

    // 24: /problems/swap-nodes-in-pairs/
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let mut prev = &mut dummy;
        while prev.next.is_some() && prev.next.as_ref()?.next.is_some() {
            let mut first = prev.next.take()?;
            let mut second = first.next.take()?;
            first.next = second.next.take();
            second.next = Some(first);
            prev.next = Some(second);
            prev = prev.next.as_mut()?.next.as_mut()?;
        }
        dummy.next
    }

    // 26: /problems/remove-duplicates-from-sorted-array/
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut next_new = 0;
        for i in 0..nums.len() {
            if i == 0 || nums[i] != nums[i - 1] {
                nums[next_new] = nums[i];
                next_new += 1;
            }
        }
        next_new as i32
    }

    // 27: /problems/remove-element/
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }

    // 28: /problems/find-the-index-of-the-first-occurrence-in-a-string/
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            None => -1,
            Some(index) => index as i32
        }
    }

    // 29: /problems/divide-two-integers/
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 { return i32::MAX; }
        let diff_sign = (dividend < 0) ^ (divisor < 0);
        let mut dividend = dividend as i64;
        let mut divisor = divisor as i64;
        let mut res = 0i32;
        dividend = dividend.abs();
        divisor = divisor.abs();
        while dividend >= divisor {
            let mut temp = divisor;
            let mut multiple = 1;
            while dividend >= (temp << 1) {
                temp <<= 1;
                multiple <<= 1;
            }
            dividend -= temp;
            res += multiple;
        }
        if diff_sign { res = -res; }
        res.max(i32::MIN).min(i32::MAX)
    }
}