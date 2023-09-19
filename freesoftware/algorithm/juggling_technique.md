# Juggling Technique

Juggling technique is an efficient technique for rotating arrays. It is based on the concept of finding the greatest common divisor (GCD) of the array seize and the number os steps to rotate.


## Juggling Technique Code Template

Here is an example Python code that uses the Jugging Technique to rotate an integer array to the right by k steps.

```python

def find_gcd(a,b):
    if b==0:
        return a
    else:
        return find_gcd(b,a%b)

def rotate(nums,k):
    n=len(nums)
    k%=n
    gcd=find_gcd(n,k)

    for i in range(gcd):
        temp=nums[i]
        j=i

        while True:
            d=(j+k)%n

            if d==i:
                break
            nums[j]=nums[d]
            j=d
        nums[j]=temp
```


## Juggling Technique Problems

Rotate an array of n elements to the right by k steps. For example, with n = 7 and k = 3, the array `[1,2,3,4,5,6,7]` is rotated to `[5,6,7,1,2,3,4]`. This is kind of a sliding window problem.

[Rotate Array](https://leetcode.com/problems/rotate-array/)