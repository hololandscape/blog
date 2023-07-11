## Memory management in Go and Rust

Go and Rust are two modern programming languages that are designed to provide memory safety and efficient memory management. Both languages have their own unique approaches to memory management.

### Memory Management in Go

Go uses a garbage collector to manage memory. 

The garbage collector automatically frees memory that is no longer in use by the program. This means that the programmer does not need to manually allocate or free memory. The garbage collector runs concurrently with the program, which means that it does not pause the program to perform garbage collection. The default garbage collector in Go is a mark-and-sweep collector, which means that it marks all objects that are still in use and then sweeps away all objects that are no longer in use.

Go also provides a way to allocate memory manually using the make() and new() functions. The make() function is used to allocate memory for slices, maps, and channels, while the new() function is used to allocate memory for other types. When memory is allocated using these functions, the garbage collector is aware of the allocation and will manage the memory accordingly.

### Memory Management in Rust

Rust uses a different approach to memory management called ownership and borrowing. In Rust, every value has an owner, which is responsible for managing the memory used by the value. When a value goes out of scope, its memory is automatically freed.
Rust also provides a way to allocate memory manually using the `Box<T>` type. The `Box<T>` type is used to allocate memory on the heap, and the memory is automatically freed when the `Box<T>` goes out of scope.
In addition to `Box<T>`, Rust also provides other types for managing memory, such as `Vec<T>` for dynamically sized arrays and `Rc<T>` and `Arc<T>` for reference counting.

## Every few minutes large latency spikes in Discord's Read States service

The context in below is from Discord's blog. In Go, on cache key eviction, memory is not immediately freed. Instead, the garbage collector(GC) runs every so often to find any memory that has no reference and then frees it. In other wortds, instead of freeing immediately after the memory is out of use, memory hangs out for a bit until the garbage collector can determine it's truly out of use. During garbage collection, Go has to do a lot of work to determine what memory is free, which can slow the program down.

Go will forece a garbage collection run every [2 minutes at minimum](https://github.com/golang/go/blob/895b7c85addfffe19b66d8ca71c31799d6e55990/src/runtime/proc.go#L4481-L4486) according to the source code. In other words, if garbage collection has not run for 2 minutes, regardless of heap growth, go will still force a garbage collection.

[GC Percent](https://pkg.go.dev/runtime/debug#SetGCPercent) can help chaning the garbage collector. But it is nothing change. It tunrs out, it was because we were not allocating memory quickly enough for it to force garbage collection to happen more often.

And according to the article, the spikes were because the garbage collector needed to scan the entire LRU cache in order to determine if the memory was truly free from references. Thus, we figured a smaller LRU cache would be faster because the garbage collector would  have less to scan.

## Memory model

Rust's memory model is able to reason about memory safety threads rather than the manual cross-goroutine memory protection that was required in Go.

## Type system

Rust has a great type system with extensive support for generics, traits, and enums. Go's type system is much more limited, with no support for generics or enums.

## Credit
https://discord.com/blog/why-discord-is-switching-from-go-to-rust