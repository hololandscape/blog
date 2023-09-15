# Counter technique

The counter technique is a technique used to count the number of occurrences of a certain value in a list. It is a very useful technique that can be used to solve many problems.


## Example of implementation

```Python

from collections import Counter

def counter_technique(nums):
    counter = Counter(nums)
    for key, value in counter.items():
        if value == 1:
            return key
```


## Problem

[Find All Numbers Disappeared in an Array](https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/description/)