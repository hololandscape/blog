# 🦎 Eigenvalues of a covariance matrix

{% hint style="info" %}
The eigenvalues of a covariance matrix play a crucial role in understanding the structure and properties of the data represented by the matrix.
{% endhint %}

### Background Information

In statistics, <mark style="color:red;">**a covariance matrix is a square matrix that summarizes the pairwise covariances**</mark> between variables in a dataset. If you have a dataset with n variables, the covariance matrix will be an n x n matrix. The element in the <mark style="color:red;">**i-th row and j-th column**</mark> represents the <mark style="color:green;">**covariance**</mark> <mark style="color:red;">**between variables i and j.**</mark>

### What was it done?

Now, the eigenvalues of a covariance matrix provide information about the _<mark style="color:green;">**variability or spread**</mark>_ of the data along the principal components. <mark style="color:red;">**Principal components are**</mark> the directions in which the data vary the most. <mark style="color:green;">**The eigenvalues quantify**</mark> the amount of variance explained by each principal component.

### What do eigenvalues and eigenvectors represent?

When you calculate the eigenvalues of a covariance matrix, you are essentially <mark style="color:green;">**finding the scaling factors for each eigenvector**</mark>. The _<mark style="color:purple;">**eigenvectors**</mark>_, on the other hand, _<mark style="color:purple;">**represent the directions along which the data varies the most.**</mark>_

### The important properties of eigenvalues of a covariance matrix

1. <mark style="color:green;">**Non-negative**</mark>: Eigenvalues are always non-negative. They can be zero or positive but not negative.
2. <mark style="color:green;">**Magnitude**</mark>: The magnitude of an eigenvalue represents the amount of variance explained by the corresponding eigenvector. Larger eigenvalues indicate that the corresponding eigenvectors capture more of the data's variance.
3. <mark style="color:green;">**Ordering**</mark>: Eigenvalues are typically ordered in descending order, with the <mark style="color:purple;">**largest eigenvalue corresponding to the first principal component**</mark>, the second largest eigenvalue corresponding to the second principal component, and so on. This ordering allows us to prioritize the principal components based on the amount of variance they explain.
4. <mark style="color:green;">**Total variance**</mark>: The sum of all eigenvalues equals the total variance of the data. This property is known as the trace property, and it implies that the sum of the eigenvalues gives us a measure of the total variability in the dataset.

### Summary

Eigenvalues and eigenvectors provide a way to <mark style="color:purple;">**transform the original variables into a new set of variables**</mark> called principal components. The <mark style="color:green;">**principal components can be used**</mark> to reduce the dimensionality of the data, visualize the data in a lower-dimensional space, or identify the most important features or patterns in the dataset.
