---
description: Port of Facebook's LLaMA model in C/C++
---

# 💻 llama.cpp

{% hint style="info" %}
The main goal of llama.cpp is to run the LLaMA model using 4-bit integer quantization on a MacBook
{% endhint %}

## Overview

It uses the LLaMA model to generate predictions or outputs based on input data. And it runs <mark style="color:red;">**without any dependencies or external libraries**</mark>. This allows for <mark style="color:red;">**faster and more efficient inference on desktop CPUs**</mark>.

#### Why is the model without external libraries and any dependencies that will good performance on the desktop CPUs?

There are several reasons:

* Without external libraries and dependencies, the model can reduce the overhead associated with loading and managing these resources.
* It is designed to run on the macOS

#### Features

* Optimizing via <mark style="color:green;">**ARM NEON**</mark>, <mark style="color:green;">**Accelerate**</mark>, and <mark style="color:green;">**Metal frameworks**</mark>
* [<mark style="color:green;">**AVX, AVX2**</mark> and <mark style="color:green;">**AVX512**</mark> support for x86 architectures](ggml.md#intrinsic-avx-avx2-on-x86)
* [Mixed F16/<mark style="background-color:green;">**F32 precision**</mark>](type-for-nnc.md)
* 4,5,8 bit [<mark style="color:green;">**integer quantization**</mark>](ggml.md#integer-quantization) support
* Supports <mark style="color:green;">**OpenBLAS**</mark>/<mark style="color:red;">**Apple BLAS**</mark>/ARM Performance Lib/ATLAS/<mark style="color:orange;">**BLIS**</mark>/Intel MKL/NVHPC/ACML/SCSL/SGIMAH  and more in BLAS
* <mark style="color:red;">**cuBLAS**</mark> and CLBlast support

## Build

* make
* cmake
* zig

## [Metal](metal.md) Build

Metal allows the computation to be executed on the GPU for Apple devices

* make
  * LLAMA\_METAL=1 make
* cmake
  * cmake -DLLAMA\_METAL=ON ..

Enable GPU inference with the `--gpu-layers|-ngl`command-line argument. Any value larger than 0 will offload the computation to the GPU.

./main -m ./models/7B/ggml-model-q4\_0.bin -n 128 -ngl 1

## [BLAS](blas.md) Build

Building the program with BLAS support may lead to some performance improvements in prompt processing using batch sizes higher than 32 (the default is 512). There are currently three different implementations of it:

* **Accelerate Framework**
  * Only available on Mac PCs and it is enabled by default.
* **OpenBLAS**
  * This provides BLAS acceleration using only the CPU(Need to install OpenBLAS first)
* [**BLIS**](https://github.com/ggerganov/llama.cpp/blob/master/docs/BLIS.md)
* **Intel MKL**
* **cuBLAS**
  * This provides BLAS acceleration using the CUDA cores of your Nvidia GPU(Need the CUDA toolkit installed)
* **CLBlast**
  * OpenCL acceleration is provided by the matrix multiplication kernels from the [CLBlast](https://github.com/CNugteren/CLBlast) project and custom kernels for ggml that can generate tokens on the GPU. (Need the OpenCL SDK)
