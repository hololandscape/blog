# 🍊 Metal

## Overview

<mark style="color:red;">**Metal is Apple's own GPU framework**</mark> that allows developers to have <mark style="color:red;">**low-level access to the GPUs**</mark> on iPhones and <mark style="color:red;">**Macs**</mark> for rendering 3D graphics and high-performance computing.

It powers hardware-accelerated graphics on Apple platforms by providing a low-overhead API, and rich shading language, tight integration between graphics and compute, and an unparalleled suite of GPU profiling and debugging tools.

Metal allows the computation to be executed on the GPU for Apple devices.

### Metal Performance Shaders(MPS)

<mark style="color:red;">**It is a framework that exposes Metal machine learning APIs**</mark> and <mark style="color:red;">**can be used to accelerate machine learning tasks**</mark> on Apple devices.

MPS Graphs is a part of the MPS framework that provides a graph-based API for machine learning computations.

MPS Graph can be used to build and execute machine learning graphs on the GPU. It provides a set of operations that can be used to build ML models. such as:

* convolution
* pooling
* activation functions

<mark style="color:red;">**Metal can be used to accelerate PyTorch model training on macOS**</mark>, and the MPS backend extends the PyTorch framework, providing scripts and capabilities to set up and run operations on Mac.

The MPS framework optimizes compute performance with kernels that are fine-tuned for the unique characteristics of each Metal GPU family.

## Reference

{% embed url="https://developer.apple.com/documentation/metal/performing_calculations_on_a_gpu" %}
