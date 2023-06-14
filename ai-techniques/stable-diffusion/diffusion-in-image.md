---
description: Diffusion processing in image
---

# ⚾ Diffusion in image

## Diffusion model

{% hint style="info" %}
Its math looks very much like diffusion in physics, so it was called diffusion model.
{% endhint %}

Stable Diffusion belongs to a <mark style="color:red;">**class of deep learning models called diffusion models**</mark>. They are generative models, meaning they are designed to generate new data like what they have seen in training. In the case of Stable diffusion, the data are images.

### What can Stable Diffusion do?

Stable Diffusion is a text-to-image deep-learning model.

<figure><img src="../../.gitbook/assets/image (15).png" alt=""><figcaption><p>Stable diffusion turns text prompts into images</p></figcaption></figure>

## Training part

### Foward diffusion

<figure><img src="../../.gitbook/assets/image (45).png" alt=""><figcaption><p>Forward diffusion turns a <a href="https://arxiv.org/abs/2011.13456">photo </a>into noise.</p></figcaption></figure>

A <mark style="color:red;">**forward diffusion**</mark> process adds noise to a training image, gradually turning it into an uncharacteristic noise image. The forward process will run any cat or dog image into a noise image. _<mark style="color:red;">**Eventually, you won't be able to tell whether they are initially a dog or a cat.**</mark>_

It's like a drop of ink fell into a glass of water. The ink drop diffuses in water. After a few minutes, it randomly distributes itself throughout the water. You can no longer tell whether it initially fell at the center or near the rim.

Example of forward diffusion of a cat image

<figure><img src="../../.gitbook/assets/image (19).png" alt=""><figcaption><p>Forward diffusion of a cat image</p></figcaption></figure>

### Reverse diffusion

<figure><img src="../../.gitbook/assets/image (9).png" alt=""><figcaption><p>The reverse diffusion process recovers an image.</p></figcaption></figure>

_<mark style="color:red;">**The main idea of reverse diffusion**</mark>_ is starting from a noisy, meaningless image, it recovers a cat OR a dog image.

Reverse diffusion in latent space please see [here](stable-diffusion-model.md#reverse-diffusion-in-latent-space).

### Summary for diffusion process

Every diffusion process has two parts below

* Drift or directed motion
* Random motion

And the _<mark style="color:blue;">**reverse diffusion towards either cat or dog images but nothing in between**</mark>_. That's why the result can either be a cat or a dog.

## How training is done

{% hint style="info" %}
I agree this is a million-dollar question.
{% endhint %}

To reverse the discussion, we need to know how much noise is added to an image. The answer is using [noise predictor.](diffusion-in-image.md#noise-predictor)

### Noise predictor

_<mark style="color:red;">**A neural network model to predict the noise added. And it is a**</mark>_ [_<mark style="color:red;">**U-Net model**</mark>_](https://en.wikipedia.org/wiki/U-Net)_<mark style="color:red;">**.**</mark>_&#x20;

Here is the training process for the **noise predictor** below:

1. Pick a training image, like a photo of a cat.
2. Generate a random noise image
3. Corrupt the training image by adding this noisy image up to a certain number of steps
4. Teach <mark style="color:red;">**the noise predictor**</mark> to tell us how much noise was added. This is done by tuning its weights and showing it the correct answer.

<figure><img src="../../.gitbook/assets/image (1).png" alt=""><figcaption></figcaption></figure>

In the above picture, noise is sequentially added at each step. The noise predictor estimates the total noise added up to each step.

After training, **we have a noise predictor capable of estimating the noise added to an image.**

### Noise predictor in [reverse diffusion](diffusion-in-image.md#reverse-diffusion)

1. Generating a completely random image and ask the noise predictor to tell us the noise
2. Subtracting this estimated noise from the original image
3. Repeat this process a few times

We will get an image of either a cat or a dog.

{% hint style="info" %}
Here is no control over generating a cat or a dog's image(unconditioned). More detail for [conditioning](conditioning.md)
{% endhint %}

<figure><img src="../../.gitbook/assets/image (13).png" alt=""><figcaption><p>Reverse diffusion works by subtracting the predicted noise from the image successively</p></figcaption></figure>

## Credit

{% embed url="https://stable-diffusion-art.com/how-stable-diffusion-work/#Diffusion_model" %}
