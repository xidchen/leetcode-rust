mod leetcode;

use leetcode::Leetcode;

// 1: /problems/two-sum/
fn two_sum() {
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;
    let result: Vec<i32> = Leetcode::two_sum(nums, target);
    println!("Two sum: {:?}", result);
}

// 3: /problems/longest-substring-without-repeating-characters/
fn length_of_longest_substring() {
    let s: String = String::from("abcdefabc");
    let result: i32 = Leetcode::length_of_longest_substring(s);
    println!("Length of longest substring: {:?}", result);
}

// 5: /problems/longest-palindromic-substring/
fn longest_palindromic_substring() {
    let s: String = String::from("babad");
    let result: String = Leetcode::longest_palindromic_substring(s);
    println!("Longest palindromic substring: {:?}", result);
}

// 6: /problems/zigzag-conversion/
fn convert() {
    let s: String = String::from("PAYPALISHIRING");
    let num_rows: i32 = 3;
    let result: String = Leetcode::convert(s, num_rows);
    println!("Zigzag conversion: {:?}", result);
}

// 7: /problems/reverse-integer/
fn reverse() {
    let x: i32 = -2147483648;
    let result: i32 = Leetcode::reverse(x);
    println!("Reverse integer: {:?}", result);
}

// 8: /problems/string-to-integer-atoi/
fn my_atoi() {
    let s: String = String::from(" -273 degree ");
    let result: i32 = Leetcode::my_atoi(s);
    println!("String to integer (atoi): {:?}", result);
}

// 9: /problems/palindrome-number/
fn is_palindrome() {
    let x: i32 = 121;
    let result: bool = Leetcode::is_palindrome(x);
    println!("Is palindrome: {:?}", result);
}

// 10: /problems/regular-expression-matching/
fn is_match() {
    let s: String = String::from("aa");
    let p: String = String::from("a*");
    let result: bool = Leetcode::is_match(s, p);
    println!("Regular expression matching: {:?}", result);
}

// 11: /problems/container-with-most-water/
fn max_area() {
    let height: Vec<i32> = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let result: i32 = Leetcode::max_area(height);
    println!("Container with most water: {:?}", result);
}

fn main() {
    two_sum();
    length_of_longest_substring();
    longest_palindromic_substring();
    convert();
    reverse();
    my_atoi();
    is_palindrome();
    is_match();
    max_area();
}
