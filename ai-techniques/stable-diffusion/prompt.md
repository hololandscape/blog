# 🥠 Prompt

## Overview

Developing a process to _<mark style="color:red;">**build good prompts**</mark>_ is the first step every Stable Diffusion user tackles.

## Anatomy of a good prompt

A good prompt needs to be _<mark style="color:red;">**detailed**</mark>_ and _<mark style="color:red;">**specific**</mark>_. A good process is to look through <mark style="color:red;">**a list of keyword categories**</mark> and _<mark style="color:red;">**decide**</mark>_ whether you want to use any of them. For example, <mark style="color:orange;">subject, medium, style, artist, website, resolution, additional details, color and lighting.</mark>

### Subject

**The** subject is what you want to **see** in the image.

### Medium

Medium is the material <mark style="color:red;">**used to make artwork**</mark>.

### Style

The **style** refers to the artistic style of the image.

### Artist

Artist names are strong modifiers. They allow you to dial in the exact style using a particular artist as a reference. It is also common to use multiple artist names to blend their styles.

### Website

Using them in a prompt is a sure way to steer the image toward these styles.

### Resolution

Resolution represents how sharp and detailed the image is.

### Additional details

Additional details are _sweeteners_ added to modify an image.

### Color

You can control the overall color of the image by adding **color** **keywords.**

### Lighting

Lighting keywords can have a huge effect on how the image looks.

## Negative prompt

Using negative prompts is another great way to steer the image, but instead of putting in what you want, you put in what you don’t want. They don’t need to be objects. They can also be styles and unwanted attributes. (e.g. ugly, deformed).

The negative prompt helped the images to pop out more, making them less flat.

<mark style="color:red;">**Using negative prompts is a must for v2 models.**</mark>

<mark style="color:red;">**Here are some universal nagative prompt:**</mark>

`ugly, tiling, poorly drawn hands, poorly drawn feet, poorly drawn face, out of frame, extra limbs, disfigured, deformed, body out of frame, bad anatomy, watermark, signature, cut off, low contrast, underexposed, overexposed, bad art, beginner, amateur, distorted face, blurry, draft, grainy`

## Prompt syntaxes apply on AUTOMATIC 111 GUI

### Keyword weight

You can adjust the **weight** of a keyword by the syntax `(keyword: factor)`. `factor` is a value such that less than 1 means less important and larger than 1 means more important.

### () and \[] syntax

An equivalent way to adjust keyword strength is to use `()` and `[]`.

* `(keyword)` <mark style="color:green;">**increases**</mark> the strength of the keyword by a factor of 1.1 and is the same as `(keyword:1.1)`.&#x20;
* `[keyword]` <mark style="color:red;">**decrease**</mark> the strength by a factor of 0.9 and is the same as `(keyword:0.9)`.

<mark style="color:red;">**We can use multiple of them**</mark>, just like in Algebra… The effect is <mark style="color:red;">**multiplicative**</mark>.

* (keyword): 1.1 ((keyword)): 1.21 (((keyword))): 1.33
* \[keyword]: 0.9 \[\[keyword]]: 0.81 \[\[\[keyword]]]: 0.73

### Keyword blending

The proper term is **prompt scheduling.**&#x20;

The syntax is \[keyword1 : keyword2: factor]

* `factor` controls at which step keyword1 is switched to keyword2. It is a number between 0 and 1.

## How long can a prompt be?

&#x20;In the basic Stable Diffusion v1 model, that limit is [75 **tokens**](conditioning.md#tokenizer).&#x20;

### Prompt limit in AUTOMATIC1111

AUTOMATIC1111 has [no token limits](https://github.com/AUTOMATIC1111/stable-diffusion-webui/wiki/Features#infinite-prompt-length). If a prompt contains more than 75 tokens, the limit of the CLIP tokenizer, it will start a new chunk of another 75 tokens, so the new “limit” becomes 150. The process can continue forever or until your computer runs out of memory.

Each chunk of 75 tokens is processed independently, and the resulting representations are concatenated before feeding into Stable Diffusion’s [U-Net](conditioning.md#feeding-embeddings-to-noise-predictor).

## Credit

{% embed url="https://stable-diffusion-art.com/prompt-guide/#Keyword_weight" %}
