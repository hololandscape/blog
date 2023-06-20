---
description: Singular Value Decomposition(A mathematical technique)
---

# 📀 SVD

### What is SVD?

Singular value decomposition (SVD) is a **matrix factorization technique** that can be used to <mark style="color:green;">**reduce the dimensionality,**</mark> denoise, and visualize data. It is a fundamental technique in machine learning and data science.

It works by decomposing a matrix into three matrices:

* A diagonal matrix of singular values
* A matrix of left singular vectors
* A matrix of right singular vectors

### Formula

Given a matrix A, the SVD factorizes it into the following form:

$$
A=UΣV^T
$$

Where:

* U is an orthogonal matrix whose columns represent the left singular vectors of A.
* Σ is a diagonal matrix with non-negative values, known as singular values.
* V^T is the transpose of an orthogonal matrix V, whose columns represent the right singular vectors of A.

The singular values in <mark style="color:red;">**Σ are arranged in descending order along the diagonal.**</mark> They <mark style="color:orange;">**indicate**</mark> the importance or significance of the corresponding singular vectors in U and V^T. The <mark style="color:red;">**first singular vector pair captures the most significant pattern**</mark> or structure in the matrix, while subsequent pairs capture less important patterns.

### SVD Works Detail

The _**left**_ and _**right**_ singular vectors are the eigenvectors of the covariance matrix.

#### Singular values

The singular values are _the square roots_ of [the **eigenvalues** of the _<mark style="color:purple;">**covariance matrix**</mark>_](eigenvalues-of-a-covariance-matrix/) of the data. The eigenvalues of the covariance matrix are the variances of the data along each <mark style="color:green;">**principal component**</mark>. The singular values are arranged in descending order, with **the largest singular value corresponding to the first principal component**, the second largest singular value corresponding to the second principal component, and so on.

#### Left Singular Vectors

The left singular vectors are the eigenvectors of the _<mark style="color:purple;">**covariance**</mark>_ _<mark style="color:purple;">**matrix**</mark>_ corresponding to the singular values. The left singular vectors are arranged in the same order as the singular values. The left singular vector corresponding to the **first singular value is the direction of the first principal component**, the left singular vector corresponding to the second singular value is the direction of the second principal component, and so on.

#### Right Singular Vectors

The right singular vectors are the eigenvectors of the _<mark style="color:purple;">**covariance matrix transposed**</mark>_ corresponding to the singular values. The right singular vectors are arranged in the same order as the singular values. The right singular vector corresponding to the first singular value is the direction of the first principal component, the right singular vector corresponding to the second singular value is the direction of the second principal component, and so on.

### Reducing the dimensionality

SVD can be used to reduce the dimensionality of a dataset by projecting the data onto a _**lower-dimensional subspace**_. The subspace <mark style="color:purple;">**is spanned**</mark> by the <mark style="color:purple;">**left singular vectors**</mark> corresponding to the largest singular values. The <mark style="color:purple;">**smaller**</mark> singular values <mark style="color:purple;">**are discarded**</mark>, as they <mark style="color:purple;">**account**</mark> for <mark style="color:purple;">**less**</mark> of the variance of the data.

### Denoising a dataset

SVD can also be used to denoise a dataset. The noise in the data is typically concentrated in the directions of the smaller singular values. By discarding the smaller singular values, the noise in the data can be significantly reduced.
