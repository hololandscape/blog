# 🏪 Covariance Matrix

{% hint style="info" %}
A covariance matrix is a symmetric square matrix that summarizes the variances and covariances between variables in a dataset.
{% endhint %}

### What is its value?

It provides a measure of how two variables change together.

### Example

If you have a dataset with n variables, the covariance matrix will be an n x n matrix. The element in the i-th row and j-th column represents the covariance between variables i and j.

The <mark style="color:red;">**diagonal elements**</mark> of the covariance matrix <mark style="color:red;">**represent**</mark> the variances of the <mark style="color:red;">**individual variables**</mark>, while the <mark style="color:red;">**off-diagonal elements**</mark> represent the covariances <mark style="color:red;">**between pairs of variables**</mark>.

### Formula

Mathematically, for a dataset with variables X₁, X₂, ..., Xₙ, the covariance between variables i and j is computed as:

$$
cov(Xᵢ, Xⱼ) = E[(Xᵢ - μᵢ)(Xⱼ - μⱼ)]
$$

Where:

* E\[ ] denotes the expected value (or average)
* μᵢ and μⱼ represent the means of variables Xᵢ and Xⱼ, respectively.

### The important properties of the covariance matrix

1. <mark style="color:green;">**Symmetry**</mark>: The covariance matrix is always symmetric because cov(Xᵢ, Xⱼ) = cov(Xⱼ, Xᵢ).
2. <mark style="color:green;">**Diagonal elements**</mark>: The diagonal elements of the covariance matrix represent the variances of  the individual variables: cov(Xᵢ, Xᵢ) = var(Xᵢ).
3. <mark style="color:green;">**Positive semi-definiteness**</mark>: The covariance matrix is positive semi-definite, which means that its eigenvalues are non-negative.

### Summary

The covariance matrix is used to capture the relationships and dependencies between variables in multivariate data.

