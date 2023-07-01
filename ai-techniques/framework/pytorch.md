# pytorch

Pytorch is a framework for deep learning in using `Eager mode` rather than `Graph mode` like Tensorflow.


## Eager mode

Eager mode can be though of as a standard scripting execution methos. The deep learning framework executes each operation immediately, as it is called, line by line, like nay other piece of Pythoon code. This makes debugging and understanding the code much easier, as you can see the results of intermediate operations and see how your model behaves.

But this approach is not all sunshine and rainbows. When executing in Eager mode. Each opertion is read from memory computed then sent to memory before the next operation is handled. This significantly increases the memory bandwidth demands if heavy optimizations aren't done.

### Operator Fusion

One of the principal optimization mehods for a model executed in Eager mode is called operator fusion. Instead of writing each intermediate result to memory, operations are fused, `so multiple functions are computed in one pass to minimize memory reads/writes`. Operator fusion `improves operator dispatch, memory bandwidth, and memory size costs`.

But the downside was that PyTorch ballooned to over 2000 operators over a few years.

![](https://substackcdn.com/image/fetch/f_auto,q_auto:good,fl_progressive:steep/https%3A%2F%2Fsubstack-post-media.s3.amazonaws.com%2Fpublic%2Fimages%2F40995db9-3a8d-4917-8943-c313807d92b9_3420x1952.png)


## Graph mode

Graph mode has two phases.

The **first pahse** is the definition of a `computation graph` representing the operations to perform. A <mark style="color:red;">*computation graph*</mark> is *a series of interconnected nodes representing operations or variables, and the edges between nodes represent the data flow between them*.

The **second phase** is the deferredd execution of an optimized version of the computation graph.

This two-phase approach makes it more challenging to understand and debug our your code, as you cannot see what is happening until the end of the graph execution. This is analogous to "interpreted" vs "compiled" languages.

The Googel generative AI models are based on Jax which is the 2nd framework that comprtes with Tensorflow.

## Summary

Pytorch is increased flexbility and usability than Tensorflow.

## Credit
https://www.semianalysis.com/p/nvidiaopenaitritonpytorch?utm_source=substack&utm_medium=email