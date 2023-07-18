from __future__ import annotations


def max_sum(arr, k):
    """
    It is used to find the maxumum or minimum sum, product, etc.
    Time complexity: O(n)
    """
    n = len(arr)
    if n < k:
        print("Invalid operation")
        return -1
    window_sum = sum(arr[:k])
    max_sum = window_sum
    for i in range(n-k):
        window_sum = window_sum - arr[i] + arr[i+k]
        max_sum = max(window_sum, max_sum)
    return max_sum


if __name__ == "__main__":
    assert max_sum([100,200,300,400], 2) == 700
    assert max_sum([2,3], 4) == -1
