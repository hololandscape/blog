# Sorting

{% hint style="info" %}
A comparison sort cannot perform better than _O_(_n_ log _n_) on average.
{% endhint %}

{% hint style="info" %}
Non-comparison sorts can have better performance in certain situations, but are often limited by the range of values in the input data.
{% endhint %}

**Binary insertion sort**

```python
def insertion_sort(arr):
    """
    The time complexity is O(n^2), less efficient than advance sorting algorithms like
    quick soer and mergesort.
    """
    for i in range(1, len(arr)):
        key=arr[i]
        j=i-1
        while j>=0 and arr[j]>key:
            arr[j+1]=arr[j]
            j-=1
        arr[j+1]=key
    return arr
    
    
# implementation of binary insertion with bisect
def binary_insertion_sort(arr):
    """
    bisect moudle provides a simple and efficient way to perform binary search
    on sorted sequences. bisect_left help find the index at which a value
    should be inserted into a sorted sequence, while maintaining the order of the sequence.
    """
    for i in range(1, len(arr)):
        x=arr[i]
        j=bisect.bisect_left(arr,x,0,i)
        # this is an effectively shifts all the elements in the range [j,i-1] one
        # position to the right, to make room for the new element that will be 
        # inserted at index 'j'
        arr[j+1:i+1]=arr[j:i]
        arr[j]=x
    return arr
```

**Heap sort**

{% hint style="info" %}
Worst-case time complexity is O(n log n) and O(1) for space complexity, it an efficient sorting algortihn for large inputs.
{% endhint %}

```python
def heap_sort(arr):
    # Heapify the array
    n=len(arr)
    for i in range(n//2-1,-1,-1):
        heapify(arr, n, i)

    # Extract elements from the heap one by one
    for i in range(n-1,0,-1):
        arr[i],arr[0]=arr[0],arr[i]
        heapify(arr,i,0)
    return arr

def heapify(arr, n, i):
    """
    This function takes an array, the size of the heap n, and the index i of the
    root of the subtree being heapified.
    It recursively swaps elements in the subtree to maintain the max heap property,
    where the parent node is greater than or equal to its child nodes.
    """
    largest =i
    l=2*i+1
    r=2*i+2
    
    if l<n and arr[l]>arr[largest]:
        largest=l
    if r<n and arr[r] >arr[largest]:
        largest=r
    if largest !=i:
        arr[i], arr[largest] =arr[largest], arr[i]
        heapify(arr, n, largest)

arr=[64,25,12,22,11]
assert heap_sort(arr)== [11,12,22,25,64]
```

**Insertion sort**

O(n^2) but it still useful for small input sizes

```
def insertion_sort(arr):
    for i in range(1, len(arr)):
        key=arr[i]
        j=i-1
        while j>=0 and arr[j]>key:
            arr[j+1]=arr[j]
            j-=1
        arr[j+1]=key
    return arr

arr=[64,25,12,22,11]
assert insertion_sort(arr)==[11,12,22,25,64]
```

**Merge sort**

```python
def merge_sort(arr):
    """
    Time Complexity is O(n log n)
    It is more efficient than Insertion sort and Bubble sort,
    less than Quick sort in most case.
    """
    if len(arr)>1:
        mid=len(arr)//2
        left_half=arr[:mid]
        right_half=arr[mid:]
        
        merge_sort(left_half)
        merge_sort(right_half)
        
        i=j=k=0
        while i<len(left_half) and j<len(right_half):
            if left_half[i]<right_half[j]:
                arr[k]=left_half[i]
                i+=1
            else:
                arr[k]=right_half[j]
                j+=1
            k+=1
        while i<len(left_half):
            arr[k]=left_half[i]
            i+=1
            k+=1
        while j<len(right_half):
            arr[k]=right_half[j]
            j+=1
            k+=1
    return err
```

**Quick sort**

```python
def quick_sort(arr):
    """
    O(n log n) in the average case, O(n^2) in the worst case(only when the pivot
    selection strategy is poor)
    So, we can use a random pivot or selecting the median of these elementsthon
    """
    if len(arr)<=1:
        return arr
    else:
        pivot=arr[0]
        less=[]
        greater=[]
        for i in arr[1:]:
            if i<=pivot:
                less.append(i)
            else:
                greater.append(i)
        # recursively calls itself on the two sub-arrays
        return quick_sort(less)+[pivot]+quick_sort(greater)
```

**References**

{% embed url="https://en.wikipedia.org/wiki/Sorting_algorithm#cite_note-4" %}
