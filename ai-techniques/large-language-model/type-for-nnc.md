---
description: Type for neural network computations
---

# 🥞 Type for NNC

## Overview

F16 and F32 precision are different floating-point formats used in ML.&#x20;

F16, also known as half-precision, uses 16 bits to represent a floating-point number.

F32, also known as single-precision, uses 32 bits.

## Differences

* **Range**:
  * F16 has a smaller range than F32, it can represent smaller and larger numbers with less precision
* **Memory usage**:
  * F16 requires less memory than F32, which can be beneficial for training and deployment of larger models or training with larger mini-batches
* **Speed**
  * Some hardware can run operations in F16 faster than in F32 which can result in faster training and inference times.

## Conclusion

In general, F32 is the standard type for neural network computations, but there is a trend in deep learning towards using F16 instead of F32 because lower precision calculations seem to be not critical for neural networks. However, some computations, such as large reductions, should still be left in F32 for numeric stability.

## Reference

{% embed url="https://moocaholic.medium.com/fp64-fp32-fp16-bfloat16-tf32-and-other-members-of-the-zoo-a1ca7897d407" %}

{% embed url="https://www.tensorflow.org/guide/mixed_precision" %}
