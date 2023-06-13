# 🤑 Stable Diffusion model

## Overview

The _<mark style="color:red;">**stable diffusion model in image space**</mark>_ is slow because the image space is enormous. For example, one 512x512 image with three color channels is a 786,432-dimensional space (Do not say that many values for ONE image). So, [the latent diffusion model](stable-diffusion-model.md#latent-diffusion-model) is used to solve this.

## Latent diffusion model

_<mark style="color:green;">**Stable Diffusion is designed to solve the speed problem.**</mark>_

_<mark style="color:red;">**Stable Diffusion is a latent diffusion model**</mark>_. Instead of operating in the high-dimensional image space, it _<mark style="color:red;">**first compresses the image into the latent space**</mark>_. The latent space is 48 times smaller, so it reaps the benefits of crunching a lot fewer numbers. That's why it's a lot faster.

## Image resolution

The image resolution is reflected in the size of the latent image tensor. <mark style="color:red;">**The size of the latent image is 4x64x64 for 512×512 images only**</mark>. <mark style="color:red;">**It is 4x96x64 for a 768×512 portrait image**</mark>. That's why it takes longer and moves <mark style="color:red;">**VRAM**</mark> to generate a larger image.

Since Stable Diffusion v1 is fine-tuned on 512x512 images, generating images larger than 512x512 could result in duplicate objects. For example, the infamous two heads, etc. If you must, keep at least one side to 512 pixels and use an AI upscaler for higher resolution.

## [Manifold hypothesis](https://en.wikipedia.org/wiki/Manifold\_hypothesis)

The high dimensionality of images is artifactual. _<mark style="color:purple;">**Natural images can be readily compressed into the much smaller latent space**</mark>_ without losing any information. This is called the [manifold hypothesis](https://en.wikipedia.org/wiki/Manifold\_hypothesis) in machine learning.

### Why is latent space possible?

[VAE](stable-diffusion-model.md#what-is-a-vae-file) can compress an image into a much smaller latent space without losing information. The reason is that natural images are not random. They are highly regular.

## [Reverse diffusion](diffusion-in-image.md#reverse-diffusion) in [latent space](vae.md#variational-autoencoder)

Here's how latent reverse diffusion in Stable Diffusion works.

1. A random latent space matrix is generated
2. The [**noise predictor**](diffusion-in-image.md#noise-predictor) estimates the noise of the latent matrix.
3. The estimated noise is then subtracted from the latent matrix.
4. Steps 2 and 3 are repeated up to specific sampling steps.
5. The **decoder** of VAE converts the latent matrix to the final image.

## What is a [VAE](vae.md) file?

VAE files are used in Stable Diffusion v1 to improve eyes and faces. They are the _<mark style="color:red;">**decoder of the autoencoder**</mark>_ we just talked about. By further fine-tuning the decoder, the model can paint finer details.&#x20;

However, _<mark style="color:red;">**compressing an image into the latent space does lose information since the original VAE did not recover the fine details**</mark>_. Instead, the VAE decoder is responsible for painting fine details.
