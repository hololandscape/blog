# A prefix sum technique

A prefix sum technique is a method for calculating the sum of a list of numbers. It is a useful primitive in `parallel algorithms` and is used as a building block in many algorithms.


## Prefix Sum

A prefix sum is a sequence of partial sums of a given sequence.

### Using accumulate

```Python
from itertools import accumulate

nums=[1,2,3,4,5]
prefix_sum = list(accumulate(nums))
print(prefix_sum)
```

### Using for loop

```Python
nums=[1,2,3,4,5]
prefix_sum = [sum(nums[:i+1]) for i in range(len(nums))]
print(prefix_sum)
```


## Questions

[Find Pivot Index](https://leetcode.com/problems/find-pivot-index/)