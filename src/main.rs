mod leetcode;

use leetcode::Leetcode;

fn main() {
    two_sum();
    length_of_longest_substring();
    longest_palindromic_substring();
}

// 1: /problems/two-sum/
fn two_sum() {
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;
    let result: Vec<i32> = Leetcode::two_sum(nums, target);
    println!("Two sum: {:?}", result);
}

// 3: /problems/longest-substring-without-repeating-characters/
fn length_of_longest_substring() {
    let s:String = String::from("abcdefabc");
    let result: i32 = Leetcode::length_of_longest_substring(s);
    println!("Length of longest substring: {:?}", result);
}

// 5: /problems/longest-palindromic-substring/
fn longest_palindromic_substring() {
    let s:String = String::from("babad");
    let result:String = Leetcode::longest_palindromic_substring(s);
    println!("Longest palindromic substring: {:?}", result);
}