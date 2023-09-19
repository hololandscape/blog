# Two-pointer Technique

It is one of the main techniques used for `in-place` Array algorithms. We can use it to iterate over the Array in two different places at the same time. 

And it is also used in the 

* Slow and Fast pointer technique
* [Sliding Window technique](./sliding_window_technique.md)

## Example

### Reverse an Array

```python
def reverse_array(arr):
    left = 0
    right = len(arr) - 1
    while left < right:
        arr[left], arr[right] = arr[right], arr[left]
        left += 1
        right -= 1
```

### Remove Duplicates from Sorted Array

```python
def remove_duplicates(arr):
    if len(arr) == 0:
        return 0
    left = 0
    for right in range(1, len(arr)):
        if arr[left] != arr[right]:
            left += 1
            arr[left] = arr[right]
    return left + 1
```

## Practice

* [Reverse String](https://leetcode.com/problems/reverse-string/)
* [Reverse Vowels of a String](https://leetcode.com/problems/reverse-vowels-of-a-string/)
* [Remove Duplicates from Sorted Array](https://leetcode.com/problems/remove-duplicates-from-sorted-array/)