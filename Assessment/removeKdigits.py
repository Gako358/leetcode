class Solution:
    def removeKdigits(self, num: str, k: int) -> str:
        if len(num) == k:
            return "0"
        stack = []
        for i in range(len(num)):
            while k and stack and stack[-1] > num[i]:
                stack.pop()
                k -= 1
            stack.append(num[i])
        while k:
            stack.pop()
            k -= 1
        return str(int("".join(stack)))


if __name__ == "__main__":
    num = "1432219"
    k = 3
    s = Solution()
    print(s.removeKdigits(num, k))
