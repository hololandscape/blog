# Sliding Window Technique

Sliding Window Technique is a method for finding subarray or substring of a given array or string in linear time O(n) by using [two pointers technique](./two_pointer_technique.md).


## Sliding Window Code Template

```python
def sliding_window(s):
    left, right = 0, 0
    while right < len(s):
        # increase right pointer
        right += 1
        # update window
        ...
        while window needs shrink:
            # increase left pointer
            left += 1
            # update window
            ...
```

## Sliding Window Technique Problems

[Max Consecutive Ones](https://leetcode.com/problems/max-consecutive-ones/)
