class Solution:
    def hammingWeight(self, n: int) -> int:
        return bin(n).count('1')


if __name__ == '__main__':
    n = 0b00000000000000000000000000001011
    print(Solution().hammingWeight(n))
