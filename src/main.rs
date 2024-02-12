mod leetcode;

fn main() {
    two_sum();
    length_of_longest_substring();
}

fn two_sum() {
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;
    let result: Vec<i32> = leetcode::two_sum(nums, target);
    println!("Two sum: {:?}", result);
}

fn length_of_longest_substring() {
    let s:String = String::from("abcdefabc");
    let result: i32 = leetcode::length_of_longest_substring(s);
    println!("Length of longest substring: {:?}", result);
}