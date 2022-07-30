"""
Given two non-empty linked lists representing two non-negative integers,
The digits are stored in reverse order, and each of their nodes contain a single digit.
Add the two numbers and return it as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.
"""

from typing import Optional, List

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        if l1 is None:
            return l2
        if l2 is None:
            return l1

        carry = 0
        head = ListNode(0)
        curr = head
        while l1 is not None or l2 is not None:
            if l1 is not None:
                carry += l1.val
                l1 = l1.next
            if l2 is not None:
                carry += l2.val
                l2 = l2.next
            curr.next = ListNode(carry % 10)
            curr = curr.next
            carry //= 10
        if carry > 0:
            curr.next = ListNode(carry)
        return head.next

if __name__ == '__main__':
    l1 = ListNode(1)
    l1.next = ListNode(2)
    l1.next.next = ListNode(3)
    l2 = ListNode(4)
    l2.next = ListNode(5)
    l2.next.next = ListNode(6)
    print(Solution().addTwoNumbers(l1, l2))
    l1 = ListNode(0)
    l1.next = ListNode(0)
    l2 = ListNode(0)
    l2.next = ListNode(0)
    print(Solution().addTwoNumbers(l1, l2))
