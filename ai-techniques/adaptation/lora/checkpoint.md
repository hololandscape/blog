# 🛫 Checkpoint

{% hint style="info" %}
In the context of training and fine-tuning large language models, a checkpoint refers to a snapshot of the model's parameters and training state at a particular point in time. It serves as a saved version of the model that can be loaded and resumed later for further training, evaluation, or deployment.
{% endhint %}



### Typically user case

During the training process, checkpoints are typically saved periodically, often after a certain number of training iterations or epochs.&#x20;

Saving checkpoints allows you to track the progress of the model over time and enables you to recover the model from a specific point in case of any interruptions or failures during training.



### Steps of doing a typical checkpoint

1. <mark style="color:green;">**Model parameters**</mark>: The values of the model's weights and biases at the time of saving the checkpoint. These parameters represent the learned information and knowledge of the model.
2. <mark style="color:green;">**Optimizer state**</mark>: The state of the optimizer, including any momentum or moving average variables associated with it. This information is crucial for resuming training from the same point and maintaining the optimization process.
3. <mark style="color:green;">**Training progress information**</mark>: Additional information related to the training process, such as the current iteration/epoch, the learning rate, loss values, and evaluation metrics. These details help in tracking the progress and performance of the model.

By saving checkpoints at regular intervals during training, you can resume training from a specific point without having to start from scratch. This is particularly useful for long and computationally expensive training processes, where interruptions, system failures, or the need for fine-tuning may arise.



### The suggestions for using the LoRA technique

In the context of low-rank adaptation of large language models, saving checkpoints allows you to capture the state of the adapted model, including the reduced-rank weight matrices and any fine-tuning progress. You can save the adapted model checkpoints for later use during evaluation or deployment.

When working with low-rank adaptation techniques, it is advisable to save checkpoints <mark style="color:green;">**at key stages**</mark>, such as <mark style="color:green;">**after applying Singular Value Decomposition**</mark>, <mark style="color:green;">**after rank reduction**</mark>, and <mark style="color:green;">**after fine-tuning**</mark>. This ensures that you have access to the intermediate versions of the model, which can be helpful for comparison, analysis, or future modifications.





