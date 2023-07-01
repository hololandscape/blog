# Time components in ML training time

There are two major time componenets in a ML model's training time:

1. `Compute(FLOPS)`: Running dense matrix multiplication within each layer
2. `Memory(Bandwidth)`: Waiting for data or layer weights to get to the compute resources. common examples of bandwidth-constrained operations are various nomalizations, pointwise operations, Softmax, and ReLU.

## Compute time

In the past, the dominant factor in ML training time was compute time, `waiting for matrix multiplies`. As GPUs continued to develop, this quickly faded away from being the primary concern.(when 2018 the BERT model was state-of-the-art, the matrix multiplication was no longer the primary factor for improving a model's performance)


## The Memory wall

As models continue to soar in size, large language models take 100s gigabytes, if not terabytes, for the model weights alone. A huge chunk of the time in large model training is not spent computing matrix multiples, but rather `waiting for data to get to the compute resources`. The obvious question is why don't architects put more memory closer to the compute. The answer is $$$.

![](https://substackcdn.com/image/fetch/f_auto,q_auto:good,fl_progressive:steep/https%3A%2F%2Fsubstack-post-media.s3.amazonaws.com%2Fpublic%2Fimages%2Fa62c2392-d588-4fdf-8872-e51ba3335250_704x513.jpeg)

**SRAM** stands for `Static Random Access Memory`. It is a type of memory that is faster and more reliable than the more common **DRAM**(`Dynamic Random Access Memory`). SRAM is used for a computer's cache and is more expensive than DRAM. SRAM is faster because it does not have to pause and wait for data to be retrieved from slower media.

The nearest shared memory pool is on the same chip and is generally made of SRAM. If we attempt to utilize huge pools of SARM to hold model weights, the cost of the chip would be prohibitive. For example, $2500,000 wafer scale chips only have 40GB of SRAM. This is not enough to hold the weights of a 100B+ parameter model.


## Operations

### normalization

Applies layer Normalization over a mini-batch of inputs as described in the paper [Layer Normalization](https://arxiv.org/abs/1607.06450).

### pointwise

Pointwise operations are operations that are applied to each element in a tensor. For example, ReLU, Sigmoid, and Tanh are all pointwise operations.

### flash-attention

It is stands for Fast and Memory-Efficient Exact Attention with IO-Awareness. FlashAttention is a neural network algorithm that aims to make attention algorithms IO-aware, accounting for reads and writes between levels of GPU memory. 

FlashAttention uses tiling to reduce the number of memory reads/writes between GPU high bandwidth memory (HBM) and GPU on-chip SRAM, and it analyzes the IO complexity of the algorithm.

## Credit
https://www.semianalysis.com/p/nvidiaopenaitritonpytorch?utm_source=substack&utm_medium=email