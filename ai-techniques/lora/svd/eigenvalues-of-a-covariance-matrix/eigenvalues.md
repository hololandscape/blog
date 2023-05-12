# 🧧 Eigenvalues

### What is it?

Eigenvalues are a fundamental concept in linear algebra.

### Formula

Given a square matrix A, an eigenvalue is a scalar λ that satisfies the equation:

$$
A*v=λ * v
$$

Here:

* <mark style="color:red;">v is a non-zero vector called the eigenvector associated with the eigenvalue λ.</mark> In other words, when <mark style="color:green;">**a matrix A operates on its eigenvector, the result is a scaled version of the eigenvector itself.**</mark>

### The important properties of eigenvalues and eigenvectors

1. <mark style="color:green;">**Multiplicity**</mark>: An eigenvalue may have a multiplicity greater than 1, which indicates that there are multiple linearly independent eigenvectors associated with that eigenvalue.
2. <mark style="color:green;">**Eigenvalue equation**</mark>: The equation A \* v = λ \* v can be rewritten as (A - λI) \* v = 0, where I is the identity matrix. This equation implies that the matrix (A - λI) is singular, i.e., it has a determinant of zero.
3. <mark style="color:green;">**Determinant and trace**</mark>: The sum of the eigenvalues of a matrix A is equal to its trace (the sum of the diagonal elements) and the product of its eigenvalues is equal to its determinant.
4. <mark style="color:green;">**Diagonalizability**</mark>: A matrix A is said to be diagonalizable if it has a full set of linearly independent eigenvectors. In this case, A can be expressed as A = P \* D \* P^(-1), where P is a matrix consisting of the eigenvectors of A, and D is a diagonal matrix with the corresponding eigenvalues on its diagonal.

### Eigenvalues have numerous applications in various fields

* In quantum mechanics, eigenvalues and eigenvectors play a fundamental role in determining the possible energy levels and corresponding states of a quantum system.
* Eigenvalue decomposition is used in image processing for tasks like image compression and denoising.
