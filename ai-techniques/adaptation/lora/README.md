---
description: >-
  Low-Rank Adaptation of Large Language Models. It is a promising technique for
  adapting LLMs to specific tasks in a computationally efficient manner.
---

# 🎁 LoRA

### What is it?

LoRA is a low-rank approximation algorithm that uses SVD to reduce the dimensionality of a dataset. It works by first computing the SVD of the data matrix. Then, it selects a subset of the singular values and corresponding singular vectors to form a low-rank approximation of the data matrix.

### Why here it used SVD?

[Introduce SVD](svd/)

### Why it is helpful for the training of a personal AI model&#x20;

LoRA is a technique for adapting large language models (LLMs) to specific tasks. LoRA reduces the number of trainable parameters in an LLM by learning pairs of `rank-decomposition matrices` while `freezing` the original weights. This vastly reduces the storage requirement for LLMs adapted to specific tasks and enables efficient task-switching during deployment all without introducing inference latency. LoRA also outperforms several other adaptation methods including `adapter`, `prefix-tuning`, and `fine-tuning`.

It works by first `pre-training` an LLM on a large corpus of text. Once the LLM is pre-trained, LoRA uses a technique called rank decomposition to learn a set of low-rank matrices that can be used to adapt the LLM to a specific task.

The rank decomposition process is performed by first computing the `singular value decomposition` (SVD) of the LLM's weights. The SVD decomposes the weights into a set of `left singular vectors`, a set of `right singular vectors`, and `a diagonal matrix of singular values`. The left and right singular vectors are then used to construct the `low-rank matrices`.

Once the `low-rank matrices` are constructed, they are used to adapt the LLM to a specific task. This is done by multiplying the LLM's weights by the low-rank matrices. The multiplication of the weights by the low-rank matrices has the effect of reducing the number of trainable parameters in the LLM.

### Sub-techniques

* [x] [**Singular Value Decomposition**](svd/)
* [x] [**Matrix Factorization**](matrix-factorization.md)
* [x] [**Checkpoint**](checkpoint.md)



### Python Code implementation for LoRA

[**https://github.com/microsoft/LoRA**](https://github.com/microsoft/LoRA)

### Paper(PDF)

[https://arxiv.org/abs/2106.09685](https://arxiv.org/abs/2106.09685)





