mod leetcode;

fn main() {
    two_sum();
}

fn two_sum() {
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;
    let result: Vec<i32> = leetcode::two_sum(nums, target);
    println!("Two sum: {:?}", result)
}
