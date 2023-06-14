# 💥 Conditioning

## Overview

The purpose of _<mark style="color:blue;">**conditioning**</mark>_ is to _<mark style="color:red;">**steer the noise predictor so that the predicted noise will give us what we want**</mark>_ after subtracting from the image.

## Text conditioning

{% hint style="info" %}
Text-to-image
{% endhint %}

#### The overview of a text prompt is processed and fed into the noise predictor

<figure><img src="../../.gitbook/assets/image (14).png" alt=""><figcaption><p>How the text prompt is processed and fed into the noise predictor to steer image generation</p></figcaption></figure>

#### [Tokenizer](conditioning.md#tokenizer-1)

<mark style="color:green;">**Tokenizer**</mark> first converts each word in the prompt to a number called a <mark style="color:green;">**token**</mark>.&#x20;

#### Token

Each token is then converted to a 768-value vector called <mark style="color:green;">**embedding**</mark>.

#### [Embedding](conditioning.md#embedding-1)

The embeddings are then processed by the <mark style="color:green;">**text transformer**</mark> and are ready to be consumed by the noise predictor.

#### Example

The tokens and embeddings of any prompt with the [notebook](https://colab.research.google.com/github/sagiodev/stablediffusion\_webui/blob/master/Stable\_Diffusion\_tokenizer\_and\_embedding\_SDA.ipynb).

### Tokenizer

<figure><img src="../../.gitbook/assets/image (51).png" alt=""><figcaption><p>Tokenizer</p></figcaption></figure>

The text prompt is first **tokenized** by a [CLIP tokenizer](https://huggingface.co/docs/transformers/model\_doc/clip) (More detail for [CLIP](clip.md)). _<mark style="color:green;">**Tokenization**</mark>_ is the computer's way of understanding words(It compares to humans). This is the reason the words in a text prompt are first converted to numbers.

_<mark style="color:red;">**A tokenizer can only tokenize words it has seen during training.**</mark>_ For example, there are "dream" and "beach" in the CLIP model but not "dreambeach". Tokenizer would break up the word "dreambeach" into two tokens "dream" and "beach". _<mark style="color:red;">**So one word does not always mean one token.**</mark>_

_<mark style="color:red;">**The stable Diffusion model is limited to using 75 tokens in a prompt. (So, it is not the same as 75 words.)**</mark>_

### Embedding

<figure><img src="../../.gitbook/assets/image (2).png" alt=""><figcaption><p>Embedding</p></figcaption></figure>

Stable diffusion v1 uses Open AI's [ViT-L/14](https://github.com/CompVis/stable-diffusion) Clip model. Embedding is a 768-value vector. Each token has its own unique embedding vector. Embedding is fixed by the CLIP model, which is learned during training.

#### Why do we need embedding?

It's because _<mark style="color:red;">**some words are closely related to each other**</mark>_. We want to take advantage of this information. For example, the embeddings of man, gentleman, and guy are nearly identical because they can be used interchangeably. Monet, Manet, and Degas all painted in impressionist styles but in different ways. The names have close but not identical embeddings.

Embedding can trigger a style with a keyword. Embeddings can do magic. Scientists have shown that finding the proper embeddings can trigger arbitrary objects and styles, a fine-tuning technique called [textual inversion](https://textual-inversion.github.io).

### Feeding embeddings to noise predictor

<figure><img src="../../.gitbook/assets/image (26).png" alt=""><figcaption><p>From embeddings to the noise predictor</p></figcaption></figure>

The embedding needs to be further processed by the _<mark style="color:red;">**text transformer**</mark>_ before feeding into the noise predictor. The transformer is like a universal adapter for conditioning. In this case, its input is <mark style="color:red;">**text embedding vectors**</mark>, but <mark style="color:purple;">**it could**</mark> as well be something else like _<mark style="color:purple;">**class labels, images**</mark>_, and [depth maps](depth-maps.md). The transformer not only further processes the data but also _<mark style="color:red;">**provides a mechanism to include different conditioning modalities**</mark>_.

### Cross-attention

The output of the text transformer is used _<mark style="color:red;">**multiple times**</mark>_ by the noise predictor throughout the U-Net. The U-Net\[#Todo] consumes it by a _<mark style="color:red;">**cross-attention mechanism**</mark>_. <mark style="color:green;">**That's where the prompt meets the image.**</mark>

Here is an example, the prompt "A man with the blue eye" as an example. Stable Diffusion pairs the two words "blue" and "eyes" together (<mark style="color:red;">**self-attention within the prompt**</mark>) so that it generates a man with blue eyes but not a man with a blue shirt. It then uses this information to <mark style="color:red;">**steer the reverse diffuse towards images containing blue eyes**</mark>. (_<mark style="color:blue;">**cross-attention between the prompt and the image**</mark>_)

## Extension

### Hypernetwork

_<mark style="color:green;">**A technique to fine-tune Stable Diffusion models**</mark>_ <mark style="color:red;">**hijacks the cross-attention networks to insert styles**</mark>. <mark style="color:blue;">**LoRA models modify the weights of the cross-attention module to change styles**</mark>. _<mark style="color:orange;">**The fact that modifying this module alone can fine-tune a Stable Diffusion model tells you how important this module is.**</mark>_

### Other conditionings

The text prompt is not the only way a Stable Diffusion model can be conditioned. Both a text prompt and a depth image are used to condition the [depth-to-image model](depth-maps.md).

[ControlNet](controlnet/) conditions the noise predictor with [detected outlines](controlnet/#edge-detection), [human poses](controlnet/#human-pose-detection), etc, and achieves excellent controls over image generations.

## Credit

{% embed url="https://stable-diffusion-art.com/how-stable-diffusion-work/#Conditioning" %}



