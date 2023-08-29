# Overview

The CS 61 series is an introduction to CS, with particular emphasis on software and on machines from a programmer's point of view.

* CS 61A concentrates on the idea of abstraction, allowing the programmer to think in terms appropriate to the problem rather than in low-level operations dictated by the computer hardware. It is about programming, such as functional programming, and object-oriented programming.

## What is Computer Science?

* What problems can be solved by computers?
* How can computers help us solve problems?
* What techniques lead to effective solutions?

## Programming Techniques

They are the techniques that can be used to solve problems.

### The Antatomy of a Recursive Function

It is a function that calls itself. The advantage of using recursive functions is that they can be used to solve complex problems with only a few lines of code. It can be used to solve problems that can be broken down into smaller, repetitive problems.

* The `def statement header` is similar to the normal functions
* Conditional statements check for `base cases`
* Base cases are evaluated `wihtout recursive calls`
* Recursive cases are evalauted `with recursive calls`


Iteration is a special case of recursion.

### Luhn Algorithm

`First`: From the rightmost digit, which is the check digit, moving left, double the value of every digit; if product of this doubling operation is greater than 9 (e.g., 7 * 2 = 14), then sum the digits of the products (e.g., 10: 1 + 0 = 1, 14: 1 + 4 = 5).
`Second`: Take the sum of all the digits.

### Converting Recursion to Iteration

`Idea`: Figure out what state must be maintained by the iterative function

### Practice

[Recursion in Python](https://leetcode.com/playground/g7ZCuvYC)