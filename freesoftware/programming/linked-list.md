# Linked List

It is a abstract data type that represents a sequence of nodes. In other words, it can be implemented in many ways. So, here is my suggestion, try to implement it according to the situation rather than only one way.


## Singly Linked List

This structure is the most common one. It is a linear data structure where each element is a separate object. Each element is called a node and every node has two parts, data and pointer to the next node. The first node is called `head` and the last node is called tail. The `tail` node points to null.


## Design a Linked List


### Design a Linked List with head and size attributes

```python
example=[1,2,3,4,5,6]
# obj = MyLinkedList()
# param_1 = obj.get(index)
# obj.addAtHead(val)
# obj.addAtTail(val)
# obj.addAtIndex(index,val)
# obj.deleteAtIndex(index)
```

```python
class Node:
    def __init__(self, val, next=None):
        self.val = val
        self.next = next

class LinkedList:
    def __init__(self):
        self.head = None
        self.size = 0
    
    def get(self, index: int) -> int:
        if index<0 or index>=self.size:
            return -1
        curr = self.head
        for _ in range(index):
            curr=curr.next
        return curr.val
    
    def addAtHead(self, val: int) -> None:
        self.head=Node(val, self.head)
        self.size += 1
    
    def addAtTail(self, val: int) -> None:
        if self.head is None:
            self.head = Node(val)
        else:
            curr=self.head
            while curr.next:
                curr=curr.next
            curr.next=Node(val)
        self.size += 1
    
    def AddAtIndex(self, index: int, val: int) -> None:
        if index<0 or index>self.size:
            return
        if index==0:
            self.addAtHead(val)
        else:
            curr=self.head
            for _ in range(index-1):
                curr=curr.next
            curr.next=Node(val, curr.next)
            self.size += 1
    
    def deleteAtIndex(self, index: int) -> None:
        if index<0 or index>=self.size:
            return
        if index==0:
            self.head=self.head.next
        else:
            curr=self.head
            for _ in range(index-1):
                curr=curr.next
            curr.next=curr.next.next
        self.size -= 1
```


### Design a Linked List with head, tail and size attributes


```python

class Node:
    def __init__(self, val, next=None):
        self.val = val
        self.next = next

class LinkedList:
    def __init__(self):
        self.head = None
        self.tail = None
        self.size = 0
    
    def get(self, index: int) -> int:
        if index<0 or index>=self.size:
            return -1
        curr = self.head
        for _ in range(index):
            curr=curr.next
        return curr.val
    
    def addAtHead(self, val: int) -> None:
        self.head = Node(val, self.head)
        if self.tail is None:
            self.tail = self.head
        self.size += 1
        
    def addAtTail(self, val: int) -> None:
        if self.tail is None:
            self.head = self.tail = Node(val)
        else:
            self.tail.next = Node(val)
            self.tail = self.tail.next
        self.size += 1
    
    def addAtIndex(self, index: int, val: int) -> None:
        if index<0 or index>self.size:
            return
        if index==0:
            self.addAtHead(val)
        elif index==self.size:
            self.addAtTail(val)
        else:
            curr = self.head
            for _ in range(index-1):
                curr = curr.next
            curr.next = Node(val, curr.next)
            self.size += 1
    
    def deleteAtIndex(self, index: int) -> None:
        if index<0 or index>=self.size:
            return
        curr = self.head
        if index==0:
            self.head = self.head.next
            if self.head is None:
                self.tail = None
        else:
            for _ in range(index-1):
                curr = curr.next
            curr.next = curr.next.next
            if index==self.size-1:
                self.tail = curr
        self.size -= 1
```

### What is the difference between the two implementations?

The first implementation is more efficient than the second one. The second implementation is more readable than the first one.

The first implementation is more efficient because it does not need to check if the tail is None or not. 
The second implementation is more readable because it does not need to check if the index is 0 or not.



## Doubly Linked List

This structure is similar to the singly linked list. The only difference is that each node has two pointers, one points to the next node and the other points to the previous node. The first node is called `head` and the last node is called `tail`. The `head` node points to null and the `tail` node points to null.

```python
class Node:
    def __init__(self, val, next=None, prev=None):
        self.val = val
        self.next = next
        self.prev = prev

class LinkedList:
    def __init__(self):
        self.head = None
        self.tail = None
        self.size = 0
    
    def get(self, index: int) -> int:
        if index<0 or index>=self.size:
            return -1
        curr = self.head
        for _ in range(index):
            curr=curr.next
        return curr.val
    
    def addAtHead(self, val: int) -> None:
        self.head = Node(val, self.head)
        if self.tail is None:
            self.tail = self.head
        else:
            self.head.next.prev = self.head
        self.size += 1
    
    def addAtTail(self, val: int) -> None:
        if self.tail is None:
            self.head = self.tail = Node(val)
        else:
            self.tail.next = Node(val, None, self.tail)
            self.tail = self.tail.next
        self.size += 1
    
    def addAtIndex(self, index: int, val: int) -> None:
        if index<0 or index>self.size:
            return
        if index==0:
            self.addAtHead(val)
        elif index==self.size:
            self.addAtTail(val)
        else:
            curr = self.head
            for _ in range(index-1):
                curr = curr.next
            curr.next = Node(val, curr.next, curr)
            curr.next.next.prev = curr.next
            self.size += 1
    
    def deleteAtIndex(self, index: int) -> None:
        if index<0 or index>=self.size:
            return
        curr = self.head
        if index==0:
            self.head = self.head.next
            if self.head is None:
                self.tail = None
            else:
                self.head.prev = None
        else:
            for _ in range(index-1):
                curr = curr.next
            curr.next = curr.next.next
            if index==self.size-1:
                self.tail = curr
            else:
                curr.next.prev = curr
        self.size -= 1
```


## Why we need Doubly Linked List? Is it more efficient than Singly Linked List?

Doubly Linked List is useful when we need to traverse the linked list in both forward and backward directions. It provides a prev pointer in addition to the next pointer, which allows us to traverse the list in reverse order. However, it requires more memory than a Singly Linked List due to the extra prev pointer. In terms of time complexity, both types of linked lists have the same time complexity for most operations, such as insertion and deletion.


## What are the techniques to solve the linked list problems?


### [Two Pointers](../algorithms/two-pointers.md)

For exmaple, how to find the linked list middle node, or it has a cycle or not.

Here is an example of checking if the linked list has a cycle or not.

```python
# Definition for singly-linked list.
class Node:
    def __init__(self, x):
        self.val = x
        self.next = None

class Solution:
    def hasCycle(self, head: Optional[ListNode]):
        slow = fast = head
        while fast and fast.next:
            slow = slow.next
            fast = fast.next.next
            if slow == fast:
                return True
        return False

```

<!-- TODO: add more related techniques -->
<!-- ### [Dummy Node](../algorithms/dummy-node.md)

### [Recursion](../algorithms/recursion.md)

### [Stack](./stack.md)

### [Hash Table](./hash-table.md)

### [Binary Search](../algorithms/binary-search.md) -->


# Questions
* [Design Linked List](https://leetcode.com/problems/design-linked-list/)
* [Two Pointer in Linked List](https://leetcode.com/problems/linked-list-cycle/)