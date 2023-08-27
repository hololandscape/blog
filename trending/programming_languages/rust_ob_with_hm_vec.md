# Rust Ownership and Borrowing Practice with HashMap and Vec

Rust has a unique approach to memory management called ownership and borrowing. In Rust, every value has an owner, which is responsible for managing the memory used by the value. When a value goes out of scope, its memory is automatically freed. Let's use a example to practice Rust's ownership and borrowing.

# Problem Description

Given two strings `ransomNote` and `magazine`, return true if `ransomNote` can be constructed from `magazine` and false otherwise. Each letter in `magazine` can only be used once in `ransomNote`.

## Example 1

```text
Input: ransomNote = "a", magazine = "b"
Output: false
```

## Example 2

```text
Input: ransomNote = "aa", magazine = "ab"
Output: false
```

## Example 3

```text
Input: ransomNote = "aa", magazine = "aab"
Output: true
```

## Constraints

* `1 <= ransomNote.length, magazine.length <= 10^5`
* `ransomNote` and `magazine` consist of lowercase English letters.


# Solutions

According to the problem above, we want to use a container to store the characters in `magazine` and check if the characters in `randsomNote` are in the container. There are some collection types to store and retrieve data in rust, like:

* Sequence: `Vec`, `String`, `VecDeque`, `LinkedList`
* Maps: `HashMap`, `BTreeMap`
* Sets: `HashSet`, `BTreeSet`
* Misc: `BinaryHeap`

## Solution with Vec

So, here we can use a `Vec` to store the characters in `magazine`. The `vec` type is a resizable array that stores values in contiguous memory blocks. And some important features of a `Vec` are:
* A `Vec` can grow or shrink at runtime.
* Values in a `Vec` can also be fetched using a reference to the collection.  

```rust
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        // We create new Vec to store characters in magazine
        let mut container:Vec<char> = vec![];
        // storing all the characters to the container
        for i in magazine.chars(){
            container.push(i);
        }
        // checking all the characters of ransom_note, if are in container and remove it.
        for i in ransom_note.chars(){
            // retrain() will remove all the elements that are equal to i. It is not a good way to do this if you have `aaaa`.
            if let Some(pos)=container.iter().position(|&x| x==i){
                container.remove(pos);
            }else{
                return false;
            }
        }
        true
    }
}
```

Here is a more concise way to do this:

```rust
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        // we can create a iterator with chars() 
        // and get a collection base on char type with collect() 
        let mut magazine_vec:Vec<char>=magazine.chars().collect();
        // same
        for i in ransom_note.chars(){
            if let Some(pos)=magazine_vec.iter().position(|&x| x==i){
                magazine_vec.remove(pos);
            }else{
                return false;
            }
        }
        true
    }
}
```

## Solution with HashMap

We can also use a `HashMap` to store the characters in `magazine`. The `HashMap` type is a collection of key-value pairs. And some important features of a `HashMap` are:
* A `HashMap` can grow or shrink at runtime.

Because we will meet the same character in `ransomNote`, we can use the character as the key and the number of the character as the value. So, we can use a `HashMap<char, i32>` to store the characters in `magazine`. And we can use the `entry()` method to check if the key is in the `HashMap` and update the value.

```rust
use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        // create HashMap to store the element in magazine, and we need to handle same character situations. So, we need to use the format like HashMap<char, i32>
        let mut hm:HashMap<char,i32>= HashMap::new();

        for i in magazine.chars(){
            // check if i in HashMap,get_mut() expected a reference
            if let Some(value)=hm.get_mut(&i){
                // get_mut() return a reference
                // value here is a mut reference, *value can be the original element, an integer
                *value+=1;
            }else{
                hm.insert(i,1);
            }
        }

        for i in ransom_note.chars(){
            // same to above
            if let Some(value) =hm.get_mut(&i){
                *value=*value-1;
                if *value<0{
                    return false;
                }
            }else{
                return false;
            }
        }
        true
    }
}
```

Here is a more concise way to do this:
```rust
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        // We create new HashMap to store characters in magazine
        let mut container:HashMap<char, i32> = HashMap::new();
        // storing all the characters to the container
        for i in magazine.chars(){
            // entry() will return a mutable reference to the value corresponding to the key
            // and insert the key with value 0 if it is not exist,
            // then we use the deference operator * to modiy the value by adding 1 to it.
            *container.entry(i).or_insert(0)+=1;
        }
        // checking all the characters of ransom_note, if are in container and remove it.
        for i in ransom_note.chars(){
            if let Some(x)=container.get_mut(&i){
                *x-=1;
                if *x<0{
                    return false;
                }
            }else{
                return false;
            }
        }
        true
    }
}
```

# Conclusion

In my opinion, we do not need to scare the ownership and borrowing. We just need to know that we can use `&` to borrow a reference to the value and use `&mut` to borrow a mutable reference to the value. And we can use `&` and `&mut` to pass the value to a function etc. `*` to dereference a reference. In another word, it is to get the value from a reference. And we can use `*` to get the value from a mutable reference and change the value.
