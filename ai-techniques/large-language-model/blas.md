---
description: Basic Linear Algebra Subprograms
---

# 🏁 BLAS

## Overview

BLAS(Basic Linear Algebra Subprograms) is <mark style="color:red;">**a set of low-level routines for performing common linear algebra operations**</mark> such as:

* vector addition
* scalar multiplication
* dot products
* linear combinations
* matrix multiplication

## Advantages

BLAS implementations are <mark style="color:blue;">**often optimized for speed on a particular machine.**</mark>

* Vector register
* SIMD instructions

So using them can bring substantial performance benefits.

* Some performance improvements in prompt processing using batch sizes higher than 32 (the default is 512).

### Several BLAS implementations

* Apple's implementation
* OpenBLAS
* Intel's implementation
* CBLAS
  * It is a C-style interface to the BLAS routines that can be called using regular C-style calls

## Reference

https://icl.utk.edu/files/publications/2017/icl-utk-1032-2017.pdf https://en.wikipedia.org/wiki/Basic\_Linear\_Algebra\_Subprograms







