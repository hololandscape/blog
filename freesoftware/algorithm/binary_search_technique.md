# Binary Search

It is another way of searching for an Array. But it has the pre-requirements. The Array must be sorted. The time complexity of a binary search is O(log N).

Binary search is where we repeatedlly look at the middle element in the Array, and determine whether the element we are looking for must be to the left, or to the right. Each time we do this, we're able to have the number of elements we still need to search, making binary search a lot faster than linear search.

But binary search only works if the Array is sorted. If the Array is not sorted, then we have to sort it first, which takes O(N log N) time. So if we have to sort the Array first, then binary search is not faster than linear search.