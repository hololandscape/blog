---
description: VAE stands for variational autoencoder
---

# 🕠 VAE

## Variational Autoencoder

{% hint style="info" %}
It is done using a technique called the <mark style="color:red;">**variational autoencoder. (VAE file)**</mark>
{% endhint %}

The VAE neural network has two parts:

* An encoder
* A decoder

The encoder compresses an image to a lower dimensional representation in the latent space. The decoder restores the image from the latent space.

<figure><img src="../../.gitbook/assets/image (60).png" alt=""><figcaption><p>Variational autoencoder transforms the image to and from the latent space.</p></figcaption></figure>

_<mark style="color:red;">**The latent space of the Stable Diffusion model is 4x64x64, 48 times smaller than the image pixel space**</mark>_. All the _<mark style="color:green;">**forward and reverse diffusions**</mark>_ we talked about are done in the latent space.

And during training, _<mark style="color:red;">**instead of generating a noisy image, it generates a random tensor in latent space**</mark>_ (latent noise). Instead of corrupting an image with noise, it corrupts the representation of the image in latent space with the latent noise. _<mark style="color:red;">**The reason for doing that is it is a lot faster since the latent space is smaller.**</mark>_
