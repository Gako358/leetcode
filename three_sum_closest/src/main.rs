// Given an integer array nums of length n and an integer target, find three integers in nums such that the sum is closest to target.

// Return the sum of the three integers.

// You may assume that each input would have exactly one solution.

struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut res = nums[0] + nums[1] + nums[2];
        for i in 0..nums.len() - 2 {
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if sum == target {
                    return sum;
                } else if sum < target {
                    l += 1;
                } else {
                    r -= 1;
                }
                if (target - sum).abs() < (target - res).abs() {
                    res = sum;
                }
            }
        }
        res

    }
}

fn main() {
    println!("{:?}", Solution::three_sum_closest(vec![-1, 2, 1, 4], 1));
}
