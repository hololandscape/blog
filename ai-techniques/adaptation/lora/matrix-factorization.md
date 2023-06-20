# ℹ Matrix Factorization

{% hint style="info" %}
Matrix factorization is a technique for dimensionality reduction and latent factor analysis.
{% endhint %}

### How is it work?

It works by decomposing a matrix into two or more lower-dimensional matrices. The original matrix can then be reconstructed by multiplying the two lower-dimensional matrices together.

### Working process

The process of matrix factorization _**involves minimizing the error**_ between _**the reconstructed matrix**_ and the _**original matrix**_ by _<mark style="color:red;">**adjusting**</mark>_ the values in the factor matrices. This can be achieved through various optimization algorithms, such as _<mark style="color:red;">**gradient descent**</mark>_ or _<mark style="color:red;">**alternating least squares**</mark>_.

### How to implement it?

It can be performed using different methods, including [SVD](svd/), NMF, and PMF. Each method has its own characteristics and is suitable for different scenarios.

### The most common application of Matrix factorization

#### Collaborative Filtering

{% hint style="info" %}
It is used to make personalized recommendations based on the preferences and behavior of users.
{% endhint %}

In collaborative filtering, a matrix is constructed with users as rows, items as columns, and the entries representing user-item interactions (such as ratings or purchase history). Matrix factorization aims to approximate this original matrix by finding two (or more) lower-rank matrices whose product closely matches the original matrix.

#### Recommender Systems

Matrix factorization has proven to be effective in recommender systems because it can _**handle sparse data**_, _**capture latent factors**_, and provide personalized recommendations even for new or unrated items.
