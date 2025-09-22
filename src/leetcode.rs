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
        l2: Option<Box<ListNode>>,
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
            } else {
                ""
            };
            let even = if i as i32 - ml >= 0 {
                &s[i - ml as usize..=i]
            } else {
                ""
            };
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
        if y > i32::MAX as i64 {
            return 0;
        }
        let result: i32 = if negative { -y as i32 } else { y as i32 };
        result
    }

    // 8: /problems/string-to-integer-atoi/
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim_start();
        if s.is_empty() {
            return 0;
        }
        let mut chars = s.chars();
        let mut sign = 1;
        if let Some(c) = chars.next() {
            if c == '+' || c == '-' {
                sign = if c == '-' { -1 } else { 1 };
            } else if c.is_ascii_digit() {
                chars = s.chars();
            } else {
                return 0;
            }
        }
        let mut res: i64 = 0;
        for c in chars {
            if c.is_ascii_digit() {
                res = res * 10 + (c as i64 - '0' as i64);
                if res * sign > i32::MAX as i64 {
                    return i32::MAX;
                }
                if res * sign < i32::MIN as i64 {
                    return i32::MIN;
                }
            } else {
                break;
            }
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
                let first_match =
                    i < s_chars.len() && (p_chars[j] == s_chars[i] || p_chars[j] == '.');
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
                max_area,
                std::cmp::min(height[i], height[j]) * (j - i) as i32,
            );
            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }
        max_area
    }

    // 12: /problems/integer-to-roman/
    pub fn int_to_roman(num: i32) -> String {
        let mapping = vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
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
                }
                Some(&value) => {
                    integer -= value;
                }
                _ => (),
            }
        }
        integer
    }

    // 14: /problems/longest-common-prefix/
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }
        if strs.len() == 1 {
            return strs[0].clone();
        }
        let mut strs = strs;
        strs.sort();
        let mut result = String::new();
        for (a, b) in strs[0].chars().zip(strs.last().unwrap().chars()) {
            if a == b {
                result.push(a);
            } else {
                break;
            }
        }
        result
    }

    // 15: /problems/3sum
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut dic: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
        let mut res: Vec<Vec<i32>> = Vec::new();
        for &n in &nums {
            *dic.entry(n).or_insert(0) += 1;
        }
        let mut sorted_nums: Vec<i32> = dic.keys().cloned().collect();
        sorted_nums.sort_unstable();
        for (i, &x) in sorted_nums.iter().enumerate() {
            if x == 0 {
                if *dic.get(&x).unwrap_or(&0) > 2 {
                    res.push(vec![0, 0, 0]);
                }
            } else if *dic.get(&x).unwrap_or(&0) > 1 && dic.contains_key(&(-2 * x)) {
                res.push(vec![x, x, -2 * x]);
            }
            if x < 0 {
                for j in (i + 1)..sorted_nums.len() {
                    let y = sorted_nums[j];
                    let z = -x - y;
                    if z <= y {
                        break;
                    }
                    if dic.contains_key(&z) && z != y {
                        res.push(vec![x, y, z]);
                    }
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
            if nums[i] + nums[j] + nums[j + 1] >= target {
                k = j + 1;
            }
            if nums[i] + nums[k - 1] + nums[k] <= target {
                j = k - 1;
            }
            while j < k {
                let s = nums[i] + nums[j] + nums[k];
                if (target - s).abs() < (target - res).abs() {
                    res = s;
                }
                if s == target {
                    return res;
                }
                if s < target {
                    j += 1;
                }
                if s > target {
                    k -= 1;
                }
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
        ]
        .iter()
        .cloned()
        .collect();
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
            if n.len() < k as usize || t < n[0] * k || n[n.len() - 1] * k < t {
                return res;
            }
            if k == 2 {
                return two_sum(n, t);
            }
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
                if sum < t || (0 < lo && n[lo] == n[lo - 1]) {
                    lo += 1;
                } else if t < sum || (hi < n.len() - 1 && n[hi] == n[hi + 1]) {
                    hi -= 1;
                } else {
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
        let d: std::collections::HashMap<char, char> = [('(', ')'), ('[', ']'), ('{', '}')]
            .iter()
            .cloned()
            .collect();
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            if d.contains_key(&c) {
                stack.push(c);
            } else {
                if stack.is_empty() || d[&stack.pop().unwrap()] != c {
                    return false;
                }
            }
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
        while let (Some(node1), Some(node2)) = (l1.as_mut(), l2.as_mut()) {
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
            if left < n {
                backtrack(res, s.clone() + "(", left + 1, right, n);
            }
            if right < left {
                backtrack(res, s.clone() + ")", left, right + 1, n);
            }
        }

        let mut result = Vec::new();
        backtrack(&mut result, String::new(), 0, 0, n);
        result
    }

    // 23: /problems/merge-k-sorted-lists/
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = std::collections::BinaryHeap::new();
        for list in lists.into_iter().flatten() {
            heap.push(ListNodeWrapper(list));
        }
        let mut dummy = ListNode::new(0);
        let mut prev = &mut dummy;
        while let Some(ListNodeWrapper(mut node)) = heap.pop() {
            if let Some(next) = node.next.take() {
                heap.push(ListNodeWrapper(next));
            }
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

    // 25: /problems/reverse-nodes-in-k-group/
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k < 2 {
            return head;
        }
        let mut node = &head;
        for _ in 0..k {
            if node.is_none() {
                return head;
            }
            node = &node.as_ref().unwrap().next;
        }
        let remainder = Self::reverse_k_group(node.clone(), k);
        let mut prev = remainder;
        let mut current = head;
        for _ in 0..k {
            let mut c = current.take().unwrap();
            current = c.next.take();
            c.next = prev;
            prev = Some(c);
        }
        prev
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
            Some(index) => index as i32,
        }
    }

    // 29: /problems/divide-two-integers/
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }
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
        if diff_sign {
            res = -res;
        }
        res.max(i32::MIN).min(i32::MAX)
    }

    // 30: /problems/substring-with-concatenation-of-all-words/
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut res = Vec::new();
        if words.is_empty() || s.len() < words.len() * words[0].len() {
            return res;
        }
        let wc = words.len();
        let wl = words[0].len();
        let sl = s.len();
        let mut wd = std::collections::HashMap::new();
        for w in words.iter() {
            *wd.entry(w).or_insert(0) += 1;
        }
        for i in 0..wl {
            let mut start = i;
            let mut cnt = 0;
            let mut tmp_dict = std::collections::HashMap::new();
            let mut j = i;
            while j <= sl - wl {
                let word = s[j..j + wl].to_string();
                if let Some(&count) = wd.get(&word) {
                    cnt += 1;
                    *tmp_dict.entry(word.clone()).or_insert(0) += 1;
                    while tmp_dict[&word] > count {
                        let start_word = s[start..start + wl].to_string();
                        *tmp_dict.entry(start_word.clone()).or_insert(0) -= 1;
                        start += wl;
                        cnt -= 1;
                    }
                    if cnt == wc {
                        res.push(start as i32);
                        let start_word = &s[start..start + wl].to_string();
                        *tmp_dict.entry(start_word.clone()).or_insert(0) -= 1;
                        start += wl;
                        cnt -= 1;
                    }
                } else {
                    start = j + wl;
                    cnt = 0;
                    tmp_dict.clear();
                }
                j += wl;
            }
        }
        res
    }

    // 31: /problems/next-permutation/
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut i = nums.len() as isize - 2;
        while i >= 0 && nums[i as usize] >= nums[(i + 1) as usize] {
            i -= 1;
        }
        if i >= 0 {
            let mut j = nums.len() as isize - 1;
            while nums[j as usize] <= nums[i as usize] {
                j -= 1;
            }
            nums.swap(i as usize, j as usize);
        }
        nums[(i + 1) as usize..].reverse();
    }

    // 32: /problems/longest-valid-parentheses/
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut max_length = 0;
        let mut stack = vec![-1];
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                stack.push(i as i32);
            } else {
                stack.pop();
                if stack.is_empty() {
                    stack.push(i as i32);
                } else {
                    max_length = max_length.max(i as i32 - stack.last().unwrap());
                }
            }
        }
        max_length
    }

    // 33: /problems/search-in-rotated-sorted-array/
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            let mid = (left + right) / 2;
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[left] <= nums[mid] {
                if nums[left] <= target && target < nums[mid] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                if nums[mid] < target && target <= nums[right] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
        -1
    }

    // 34: /problems/find-first-and-last-position-of-element-in-sorted-array/
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        fn binary(nums: &Vec<i32>, tgt: f64, left: i32, right: i32) -> i32 {
            let mut left = left;
            let mut right = right;
            while left <= right {
                let mid = (left + right) / 2;
                if tgt > nums[mid as usize] as f64 {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            left
        }
        let lower = binary(&nums, target as f64 - 0.5, 0, nums.len() as i32 - 1);
        let upper = binary(&nums, target as f64 + 0.5, 0, nums.len() as i32 - 1);
        if lower == upper {
            vec![-1, -1]
        } else {
            vec![lower, upper - 1]
        }
    }

    // 35: /problems/search-insert-position/
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        while left <= right {
            let mid = (left + right) / 2;
            if nums[mid as usize] == target {
                return mid;
            }
            if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        left
    }

    // 36: /problems/valid-sudoku/
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: Vec<Vec<char>> = vec![vec![]; 9];
        let mut cols: Vec<Vec<char>> = vec![vec![]; 9];
        let mut boxes: Vec<Vec<char>> = vec![vec![]; 9];
        for r in 0..9 {
            for c in 0..9 {
                let digit = board[r][c];
                if digit == '.' {
                    continue;
                }
                let box_index = r / 3 * 3 + c / 3;
                if rows[r].contains(&digit)
                    || cols[c].contains(&digit)
                    || boxes[box_index].contains(&digit)
                {
                    return false;
                }
                rows[r].push(digit);
                cols[c].push(digit);
                boxes[box_index].push(digit);
            }
        }
        true
    }

    // 37: /problems/sudoku-solver/
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        fn solve(board: &mut Vec<Vec<char>>) -> bool {
            for r in 0..9 {
                for c in 0..9 {
                    if board[r][c] == '.' {
                        for digit in '1'..='9' {
                            if is_valid(board, r, c, digit) {
                                board[r][c] = digit;
                                if solve(board) {
                                    return true;
                                }
                                board[r][c] = '.';
                            }
                        }
                        return false;
                    }
                }
            }
            true
        }
        fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, digit: char) -> bool {
            // Check row
            for c in 0..9 {
                if board[row][c] == digit {
                    return false;
                }
            }
            for r in 0..9 {
                if board[r][col] == digit {
                    return false;
                }
            }
            let box_row = (row / 3) * 3;
            let box_col = (col / 3) * 3;
            for r in box_row..box_row + 3 {
                for c in box_col..box_col + 3 {
                    if board[r][c] == digit {
                        return false;
                    }
                }
            }
            true
        }
        solve(board);
    }

    // 38: /problems/count-and-say/
    pub fn count_and_say(n: i32) -> String {
        let mut sequence: Vec<i32> = vec![1];
        for _ in 0..n - 1 {
            let mut next: Vec<i32> = Vec::new();
            for &num in &sequence {
                if next.is_empty() || next[next.len() - 1] != num {
                    next.push(1);
                    next.push(num);
                } else {
                    let len = next.len();
                    next[len - 2] += 1;
                }
            }
            sequence = next;
        }
        sequence.iter().map(|&i| i.to_string()).collect()
    }

    // 39: /problems/combination-sum/
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        fn backtrack(nums: &[i32], nxt: usize, t: i32, p: &mut Vec<i32>, r: &mut Vec<Vec<i32>>) {
            if t == 0 {
                r.push(p.clone());
                return;
            }
            if nxt == nums.len() {
                return;
            }
            let mut i: i32 = 0;
            while t - i * nums[nxt] >= 0 {
                for _ in 0..i {
                    p.push(nums[nxt]);
                }
                backtrack(nums, nxt + 1, t - i * nums[nxt], p, r);
                for _ in 0..i {
                    p.pop();
                }
                i += 1;
            }
        }
        let mut p = Vec::new();
        backtrack(&candidates, 0, target, &mut p, &mut res);
        res
    }

    // 40: /problems/combination-sum-ii/
    pub fn combination_sum_2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates: Vec<i32> = candidates;
        candidates.sort_unstable();
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut current_combination: Vec<i32> = Vec::new();
        fn backtrack(s: usize, c: &[i32], t: i32, p: &mut Vec<i32>, r: &mut Vec<Vec<i32>>) {
            if t == 0 {
                r.push(p.clone());
                return;
            }
            if t < 0 {
                return;
            }
            for i in s..c.len() {
                if i > s && c[i] == c[i - 1] {
                    continue;
                }
                p.push(c[i]);
                backtrack(i + 1, c, t - c[i], p, r);
                p.pop();
            }
        }
        backtrack(0, &candidates, target, &mut current_combination, &mut res);
        res
    }

    // 41: /problems/first-missing-positive/
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        for i in 0..n {
            while nums[i] > 0 && nums[i] <= n as i32 && nums[nums[i] as usize - 1] != nums[i] {
                let pos = nums[i] as usize - 1;
                nums.swap(i, pos);
            }
        }
        for i in 0..n {
            if nums[i] != i as i32 + 1 {
                return i as i32 + 1;
            }
        }
        n as i32 + 1
    }

    // 42: /problems/trapping-rain-water/
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut left_max = height[left];
        let mut right_max = height[right];
        let mut res = 0;
        while left < right {
            if left_max < right_max {
                left += 1;
                if height[left] > left_max {
                    left_max = height[left];
                }
                res += left_max - height[left];
            } else {
                right -= 1;
                if height[right] > right_max {
                    right_max = height[right];
                }
                res += right_max - height[right];
            }
        }
        res
    }

    // 43: /problems/multiply-strings/
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }
        let num1_bytes = num1.as_bytes();
        let num2_bytes = num2.as_bytes();
        let mut res = vec![0; num1.len() + num2.len()];
        for i in (0..num1.len()).rev() {
            for j in (0..num2.len()).rev() {
                let d1 = (num1_bytes[i] - b'0') as i32;
                let d2 = (num2_bytes[j] - b'0') as i32;
                let product = d1 * d2;
                let pos1 = i + j;
                let pos2 = i + j + 1;
                let sum = product + res[pos2];
                res[pos2] = sum % 10;
                res[pos1] += sum / 10;
            }
        }
        let mut start = 0;
        while start < res.len() && res[start] == 0 {
            start += 1;
        }
        res[start..]
            .iter()
            .map(|&d| (d as u8 + b'0') as char)
            .collect()
    }

    // 44: /problems/wildcard-matching/
    pub fn is_match_wildcard(s: String, p: String) -> bool {
        let s_bytes = s.as_bytes();
        let p_bytes = p.as_bytes();
        let mut s_idx = 0;
        let mut p_idx = 0;
        let mut star_idx = None;
        let mut match_idx = 0;
        while s_idx < s_bytes.len() {
            if p_idx < p_bytes.len() && (p_bytes[p_idx] == b'?' || p_bytes[p_idx] == s_bytes[s_idx])
            {
                s_idx += 1;
                p_idx += 1;
            } else if p_idx < p_bytes.len() && p_bytes[p_idx] == b'*' {
                star_idx = Some(p_idx);
                match_idx = s_idx;
                p_idx += 1;
            } else if let Some(star_pos) = star_idx {
                p_idx = star_pos + 1;
                match_idx += 1;
                s_idx = match_idx;
            } else {
                return false;
            }
        }
        while p_idx < p_bytes.len() && p_bytes[p_idx] == b'*' {
            p_idx += 1;
        }
        p_idx == p_bytes.len()
    }

    // 45: /problems/jump-game-ii/
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }
        let mut jumps = 0;
        let mut current_end = 0;
        let mut farthest = 0;
        for i in 0..n - 1 {
            farthest = farthest.max(i + nums[i] as usize);
            if i == current_end {
                jumps += 1;
                current_end = farthest;
                if current_end >= n - 1 {
                    break;
                }
            }
        }
        jumps
    }

    // 46: /problems/permutations/
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut nums = nums;
        fn backtrack(nums: &mut Vec<i32>, start: usize, r: &mut Vec<Vec<i32>>) {
            if start == nums.len() {
                r.push(nums.clone());
                return;
            }
            for i in start..nums.len() {
                nums.swap(start, i);
                backtrack(nums, start + 1, r);
                nums.swap(start, i);
            }
        }
        backtrack(&mut nums, 0, &mut res);
        res
    }

    // 47: /problems/permutations-ii/
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut nums = nums;
        fn backtrack(nums: &mut Vec<i32>, start: usize, r: &mut Vec<Vec<i32>>) {
            let mut used = std::collections::HashSet::new();
            if start == nums.len() {
                r.push(nums.clone());
                return;
            }
            for i in start..nums.len() {
                if used.contains(&nums[i]) {
                    continue;
                }
                used.insert(nums[i]);
                nums.swap(start, i);
                backtrack(nums, start + 1, r);
                nums.swap(start, i);
            }
        }
        backtrack(&mut nums, 0, &mut res);
        res
    }

    // 48: /problems/rotate-image/
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n {
            for j in i + 1..n {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }
        for i in 0..n {
            matrix[i].reverse();
        }
    }

    // 49: /problems/group-anagrams/
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut res = std::collections::HashMap::new();
        for s in strs {
            let mut key = vec![0; 26];
            for c in s.chars() {
                key[c as usize - 'a' as usize] += 1;
            }
            let key = key.iter().map(|&i| i as u8).collect::<Vec<u8>>();
            let key = String::from_utf8(key).unwrap();
            let entry = res.entry(key).or_insert(Vec::new());
            entry.push(s);
        }
        res.into_iter().map(|(_, v)| v).collect()
    }

    // 50: /problems/powx-n/
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        let mut base = x;
        let mut exp = n as i64;
        if exp < 0 {
            base = 1.0 / base;
            exp = -exp;
        }
        let mut res = 1.0;
        while exp > 0 {
            if exp % 2 == 1 {
                res *= base;
            }
            base *= base;
            exp /= 2;
        }
        res
    }

    // 51: /problems/n-queens/
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut res = Vec::new();
        let mut queens = vec![0; n as usize];
        fn backtrack(
            row: usize,
            n: usize,
            cols: u32,
            diag1: u32,
            diag2: u32,
            queens: &mut Vec<usize>,
            res: &mut Vec<Vec<String>>,
        ) {
            if row == n {
                let mut solution = vec![vec!['.'; n]; n];
                for (r, &c) in queens.iter().enumerate() {
                    solution[r][c] = 'Q';
                }
                let solution: Vec<String> = solution
                    .iter()
                    .map(|row| row.iter().collect())
                    .collect();
                res.push(solution);
                return;
            }
            let mut available = !(cols | diag1 | diag2) & ((1 << n) - 1);
            while available != 0 {
                let pos = available & (!available + 1);
                let col = pos.trailing_zeros() as usize;
                queens[row] = col;
                backtrack(
                    row + 1,
                    n,
                    cols | pos,
                    (diag1 | pos) << 1,
                    (diag2 | pos) >> 1,
                    queens,
                    res,
                );
                available &= available - 1;
            }
        }
        backtrack(0, n as usize, 0, 0, 0, &mut queens, &mut res);
        res
    }

    // 52: /problems/n-queens-ii/
    pub fn total_n_queens(n: i32) -> i32 {
        fn backtrack(
            row: usize,
            n: usize,
            cols: u32,
            diag1: u32,
            diag2: u32,
        ) -> i32 {
            if row == n {
                return 1;
            }
            let mut count = 0;
            let mut available = !(cols | diag1 | diag2) & ((1 << n) - 1);
            while available != 0 {
                let pos = available & (!available + 1);
                count += backtrack(
                    row + 1,
                    n,
                    cols | pos,
                    (diag1 | pos) << 1,
                    (diag2 | pos) >> 1
                );
                available &= available - 1;
            }
            count
        }
        backtrack(0, n as usize, 0, 0, 0)
    }

    // 53: /problems/maximum-subarray/
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut current_max = nums[0];
        let mut global_max = nums[0];
        for &num in nums.iter().skip(1) {
            if current_max > 0 {
                current_max += num;
            } else {
                current_max = num;
            }
            if current_max > global_max {
                global_max = current_max;
            }
        }
        global_max
    }

    // 54: /problems/spiral-matrix/
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut matrix = matrix;
        while !matrix.is_empty() {
            res.extend(matrix.remove(0));
            if matrix.is_empty() {
                break;
            }
            let rows = matrix.len();
            let cols = matrix[0].len();
            if cols == 0 {
                break;
            }
            let mut rotated = vec![vec![0; rows]; cols];
            for i in 0..rows {
                for j in 0..cols {
                    rotated[cols - j - 1][i] = matrix[i][j];
                }
            }
            matrix = rotated;
        }
        res
    }

    // 55: /problems/jump-game/
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reach = 0;
        for i in 0..nums.len() {
            if i > max_reach {
                return false;
            }
            max_reach = max_reach.max(i + nums[i] as usize);
        }
        if max_reach >= nums.len() - 1 {
            return true;
        }
        true
    }
}
