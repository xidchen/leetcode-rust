mod leetcode;
mod structs;

use leetcode::Leetcode;
use structs::ListNode;

fn list_to_linked_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy: Box<ListNode> = Box::new(ListNode::new(0));
    let mut current: &mut Box<ListNode> = &mut dummy;
    for num in nums {
        current.next = Some(Box::new(ListNode::new(num)));
        current = current.next.as_mut()?;
    }
    dummy.next
}

fn linked_list_to_list(mut node: Option<Box<ListNode>>) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::new();
    while let Some(current) = node {
        nums.push(current.val);
        node = current.next;
    }
    nums
}

// 1: /problems/two-sum/
fn two_sum() {
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;
    let result: Vec<i32> = Leetcode::two_sum(nums, target);
    println!("Two sum: {:?}", result);
}

// 2: /problems/add-two-numbers/
fn add_two_numbers() {
    let l1: Option<Box<ListNode>> = list_to_linked_list(vec![9, 9, 9, 9, 9, 9, 9]);
    let l2: Option<Box<ListNode>> = list_to_linked_list(vec![9, 9, 9, 9]);
    let result_link_list: Option<Box<ListNode>> = Leetcode::add_two_numbers(l1, l2);
    let result: Vec<i32> = linked_list_to_list(result_link_list);
    println!("Adding two linked list numbers: {:?}", result)
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

// 12: /problems/integer-to-roman/
fn int_to_roman() {
    let num: i32 = 2024;
    let result: String = Leetcode::int_to_roman(num);
    println!("Integer to roman: {:?}", result);
}

// 13: /problems/roman-to-integer/
fn roman_to_int() {
    let s: String = String::from("MMXXIV");
    let result: i32 = Leetcode::roman_to_int(s);
    println!("Roman to integer: {:?}", result);
}

// 14: /problems/longest-common-prefix/
fn longest_common_prefix() {
    let strs: Vec<String> = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    let result: String = Leetcode::longest_common_prefix(strs);
    println!("Longest common prefix: {:?}", result);
}

// 15: /problems/3sum
fn three_sum() {
    let nums: Vec<i32> = vec![-1, 0, 1, 2, -1, -4];
    let result: Vec<Vec<i32>> = Leetcode::three_sum(nums);
    println!("Three sum: {:?}", result);
}

// 16: /problems/3sum-closest/
fn three_sum_closest() {
    let nums: Vec<i32> = vec![4, 0, 5, -5, 3, 3, 0, -4, -5];
    let target: i32 = -2;
    let result: i32 = Leetcode::three_sum_closest(nums, target);
    println!("Three sum closest: {:?}", result)
}

// 17: /problems/letter-combinations-of-a-phone-number/
fn letter_combinations() {
    let digits: String = String::from("38");
    let result: Vec<String> = Leetcode::letter_combinations(digits);
    println!("Letter combination: {:?}", result);
}

// 18: /problems/4sum/
fn four_sum() {
    let nums: Vec<i32> = vec![0,0,0,1000000000,1000000000,1000000000,1000000000];
    let target: i32 = 1000000000;
    let result: Vec<Vec<i32>> = Leetcode::four_sum(nums, target);
    println!("Four sum: {:?}", result);
}

// 19: /problems/remove-nth-node-from-end-of-list/
fn remove_nth_from_end() {
    let head: Option<Box<ListNode>> = list_to_linked_list(vec![1, 2, 3, 4, 5]);
    let n: i32 = 2;
    let result_link_list: Option<Box<ListNode>> = Leetcode::remove_nth_from_end(head, n);
    let result: Vec<i32> = linked_list_to_list(result_link_list);
    println!("Remove nth node from end of list: {:?}", result);
}

// 20: /problems/valid-parentheses/
fn is_valid() {
    let s: String = String::from("()[]{}");
    let result: bool = Leetcode::is_valid(s);
    println!("Valid parenthesis: {:?}", result);
}

// 21: /problems/merge-two-sorted-lists/
fn merge_two_lists() {
    let list1: Option<Box<ListNode>> = list_to_linked_list(vec![1, 2, 4]);
    let list2: Option<Box<ListNode>> = list_to_linked_list(vec![1, 3, 4]);
    let result_link_list: Option<Box<ListNode>> = Leetcode::merge_two_lists(list1, list2);
    let result: Vec<i32> = linked_list_to_list(result_link_list);
    println!("Merge two sorted lists: {:?}", result);
}

// 22: /problems/generate-parentheses/
fn generate_parenthesis() {
    let n: i32 = 3;
    let result: Vec<String> = Leetcode::generate_parenthesis(n);
    println!("Generate parentheses: {:?}", result);
}

// 23: /problems/merge-k-sorted-lists/
fn merge_k_lists() {
    let lists: Vec<Option<Box<ListNode>>> = vec![
        list_to_linked_list(vec![1, 4, 5]),
        list_to_linked_list(vec![1, 3, 4]),
        list_to_linked_list(vec![2, 6]),
    ];
    let result_link_list: Option<Box<ListNode>> = Leetcode::merge_k_lists(lists);
    let result: Vec<i32> = linked_list_to_list(result_link_list);
    println!("Merge k sorted lists: {:?}", result)
}

// 24: /problems/swap-nodes-in-pairs/
fn swap_pairs() {
    let head: Option<Box<ListNode>> = list_to_linked_list(vec![1, 2, 3, 4, 5, 6]);
    let result_link_list: Option<Box<ListNode>> = Leetcode::swap_pairs(head);
    let result: Vec<i32> = linked_list_to_list(result_link_list);
    println!("Swap nodes in pairs: {:?}", result);
}

// 26: /problems/remove-duplicates-from-sorted-array/
fn remove_duplicates() {
    let mut nums: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let result: i32 = Leetcode::remove_duplicates(&mut nums);
    println!("Remove duplicates from sorted array: {:?}", result);
}

// 27: /problems/remove-element/
fn remove_element() {
    let mut nums: Vec<i32> = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val: i32 = 2;
    let result: i32 = Leetcode::remove_element(&mut nums, val);
    println!("Remove element: {:?}", result);
}

// 28: /problems/find-the-index-of-the-first-occurrence-in-a-string/
fn str_str() {
    let haystack: String = String::from("sadness");
    let needle: String = String::from("sad");
    let result: i32 = Leetcode::str_str(haystack, needle);
    println!("Find the index of the first occurrence in a string: {:?}", result);
}

// 29: /problems/divide-two-integers/
fn divide() {
    let dividend: i32 = 7;
    let divisor: i32 = -3;
    let result: i32 = Leetcode::divide(dividend, divisor);
    println!("Divide two integers: {:?}", result);
}

// 30: /problems/substring-with-concatenation-of-all-words/
fn find_substring() {
    let s: String = String::from("barfoothefoobarman");
    let words: Vec<String> = vec!["foo".to_string(), "bar".to_string()];
    let result: Vec<i32> = Leetcode::find_substring(s, words);
    println!("Substring with concatenation of all words: {:?}", result);
}

// 31: /problems/next-permutation/
fn next_permutation() {
    let mut result: Vec<i32> = vec![4, 5, 3, 2, 1];
    Leetcode::next_permutation(&mut result);
    println!("Next permutation: {:?}", result);
}

// 32: /problems/longest-valid-parentheses/
fn longest_valid_parentheses() {
    let s: String = String::from(")()())");
    let result: i32 = Leetcode::longest_valid_parentheses(s);
    println!("Longest valid parentheses: {:?}", result);
}

// 33: /problems/search-in-rotated-sorted-array/
fn search() {
    let nums: Vec<i32> = vec![4, 5, 6, 7, 0, 1, 2];
    let target: i32 = 0;
    let result: i32 = Leetcode::search(nums, target);
    println!("Search in rotated sorted array: {:?}", result);
}

// 34: /problems/find-first-and-last-position-of-element-in-sorted-array/
fn search_range() {
    let nums: Vec<i32> = vec![5, 7, 7, 8, 8, 10];
    let target: i32 = 8;
    let result: Vec<i32> = Leetcode::search_range(nums, target);
    println!("Find first and last position of element in sorted array: {:?}", result);
}

// 35: /problems/search-insert-position/
fn search_insert() {
    let nums: Vec<i32> = vec![1, 3, 5, 6];
    let target: i32 = 0;
    let result: i32 = Leetcode::search_insert(nums, target);
    println!("Search insert position: {:?}", result);
}

fn main() {
    two_sum();
    add_two_numbers();
    length_of_longest_substring();
    longest_palindromic_substring();
    convert();
    reverse();
    my_atoi();
    is_palindrome();
    is_match();
    max_area();
    int_to_roman();
    roman_to_int();
    longest_common_prefix();
    three_sum();
    three_sum_closest();
    letter_combinations();
    four_sum();
    remove_nth_from_end();
    is_valid();
    merge_two_lists();
    generate_parenthesis();
    merge_k_lists();
    swap_pairs();
    remove_duplicates();
    remove_element();
    str_str();
    divide();
    find_substring();
    next_permutation();
    longest_valid_parentheses();
    search();
    search_range();
    search_insert()
}
