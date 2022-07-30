"""
Given an array of integers nums and an integer target, return the indices of the two numbers 
such that they add up to target.
"""

from typing import List

class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        for i in range(len(nums)):
            for j in range(i+1, len(nums)):
                if nums[i] + nums[j] == target:
                    return [i, j]



if __name__ == '__main__':
    nums = [2, 7, 11, 15]
    target = 9
    print(Solution().twoSum(nums, target))
    # [0, 1]
    nums = [3, 2, 4]
    target = 6
    print(Solution().twoSum(nums, target))
    # [1, 2]
    nums = [3, 3]
    target = 6
    print(Solution().twoSum(nums, target))
    # [0, 1]
    nums = [3, 3]
    target = 3


