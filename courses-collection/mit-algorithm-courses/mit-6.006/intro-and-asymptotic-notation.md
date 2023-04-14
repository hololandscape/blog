# Intro and asymptotic notation

**Overview**

* What is an algorithm?
* Asymptotic notation
* X-case analysis
* Algorithm design paradigms
* Running time analysis

**What is an algorithm?**

{% hint style="info" %}
A step-by-step procedure for solving a problem or accomplishing a task.
{% endhint %}

Some of the key characteristics of an algorithm:

* **Inputs**: Algorithms typically take one or more inputs, which are the data or information that the algorithm uses to perform its task.
* **Outputs**: Algorithms produce one or more outputs, which are the results of the algorithm's computations.
* **Definiteness**: Algorithms must have well-defined and unambiguous instructions that can be followed exactly by a person or a computer.
* **Finiteness**: Algorithms must terminate after a finite number of steps.
* **Effectiveness**: Algorithms must be effective in solving the problem for which they are designed, meaning that they must produce correct and useful results.

**Asymptotic notation**

{% hint style="info" %}
Asymptotic notation is a way to describe the performance of an algorithm as its input size grows to infinity.
{% endhint %}

The most common types of asymptotic notation:

* **Big-O notation (O)**: This notation describes the upper bound on the growth rate of an algorithm. It represents the worst-case scenario for the algorithm's performance.
* **Omega notation (Ω)**: This notation describes the lower bound on the growth rate of an algorithm. It represents the best-case scenario for the algorithm's performance.
* **Theta notation (Θ)**: This notation describes both the upper and lower bounds on the growth rate of an algorithm. It represents the tightest possible bound on the algorithm's performance.

**X-case analysis**

In general, worst-case analysis is the most important type of analysis for algorithm design, since it provides an upper bound on the algorithm's performance that can be used to make guarantees about its behavior in real-world scenarios

**Algorithm design paradigms**

* Divide and conquer
* Dynamic programming
* Greedy algorithms

**Running time analysis**

{% hint style="info" %}
Running time analysis is the process of determining how the running time of an algorithm scales with respect to the size of the input.
{% endhint %}

