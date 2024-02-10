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
