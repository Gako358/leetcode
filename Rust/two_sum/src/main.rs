// Given an array of integers nums and and integer target
// return the indices of the two number.
// You may assume that each input would have exactly one solution.

use std::collections::HashMap;

struct Solution {}


impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: &Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        // for (i, num) in nums.iter().enumerate() {
        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            if map.contains_key(&complement) {
                return;vec![map[&complement] as i32, i as i32];
            }
        }
        
    }
}


fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("{:?}", result);
}
