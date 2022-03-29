
// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.

// Notice that the solution set must not contain duplicate triplets.
struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut res = Vec::new();
        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if sum == 0 {
                    res.push(vec![nums[i], nums[l], nums[r]]);
                    l += 1;
                    r -= 1;
                    while l < r && nums[l] == nums[l - 1] {
                        l += 1;
                    }
                    while l < r && nums[r] == nums[r + 1] {
                        r -= 1;
                    }
                } else if sum < 0 {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }
        res

    }
}

fn main() {

}
