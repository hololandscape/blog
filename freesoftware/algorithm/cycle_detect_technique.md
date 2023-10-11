# Cycle Detect Technique

Here we use the `slow` and `fast` pointer technique to detect a cycle in a linked list.

## How it works step by step

* Initialize two pointers `slow` and `fast` to the head of the linked list.
* Move `slow` one step forward and `fast` two steps forward.
* If `slow` and `fast` meet at some point, then there is a cycle in the linked list.
* If `slow` or `fast` reaches the end of the linked list, then there is no cycle in the linked list.


## Advantages

How to confirm the node where the cycle starts?

* Initialize two pointers `slow` and `fast` to the head of the linked list.
* Move `slow` one step forward and `fast` two steps forward.
* If `slow` and `fast` meet at some point, then there is a cycle in the linked list.
* If `slow` or `fast` reaches the end of the linked list, then there is no cycle in the linked list.
* If there is a cycle, then move `slow` to the head of the linked list.
* Move `slow` and `fast` one step forward until they meet again.
* The node where they meet is the node where the cycle starts.


## Programming questions

* [Linked List Cycle](https://leetcode.com/problems/linked-list-cycle/)
* [Linked List Cycle II](https://leetcode.com/problems/linked-list-cycle-ii/)
