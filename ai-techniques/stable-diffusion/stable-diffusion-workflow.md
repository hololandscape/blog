# 👷 Stable Diffusion workflow

## Overview

Let's go through some examples of wat happens under the hood.

## Text-to-image

We give Stable Diffusion a text prompt, and it returns an image.

### Step1

Stable Diffusion generates a _<mark style="color:red;">**random tensor in the latent space**</mark>_. We _<mark style="color:purple;">**control this tensor by setting the**</mark>_ [_<mark style="color:purple;">**seed**</mark>_ ](the-important-parameters-for-stunning-ai-image.md#seed)_<mark style="color:purple;">**of the random number generator.**</mark>_ If we <mark style="color:blue;">**set the seed to a certain value**</mark>, we will <mark style="color:blue;">**always get the same random tensor**</mark>. _<mark style="color:red;">**This is your image in latent space. But it is all noise for now.**</mark>_

<figure><img src="../../.gitbook/assets/image (51).png" alt="" width="339"><figcaption><p>A random tensor is generated in latent space</p></figcaption></figure>

### Step2

The noise predictor U-Net takes the latent noisy image and text prompt as input and predicts the noise, also in latent space (a 4x64x64 tensor).

<figure><img src="../../.gitbook/assets/image (45).png" alt=""><figcaption></figcaption></figure>

### Step3

Subtract the latent noise from the latent image. This becomes our **new latent image**.

<figure><img src="../../.gitbook/assets/image (28).png" alt=""><figcaption></figcaption></figure>

{% hint style="info" %}
Step2 and 3 are repeated for a <mark style="color:red;">**certain number of sampling steps**</mark>, for example 20 times.
{% endhint %}

### Step4

Finally, [the decoder of VAE converts the latent image back to pixel space](vae.md#variational-autoencoder). This is the image you get after running Stable Diffusion.

<figure><img src="../../.gitbook/assets/image (2).png" alt=""><figcaption></figcaption></figure>

#### The process of the image evolves in each sampling step

<figure><img src="https://i0.wp.com/stable-diffusion-art.com/wp-content/uploads/2022/12/cat_euler_15.gif?resize=512%2C512&#x26;ssl=1" alt=""><figcaption><p>Image at each sampling step</p></figcaption></figure>

## Noise schedule

The image changes from noisy to clean. _<mark style="color:red;">**How about if the noise predictor not working well in the initial steps?**</mark>_ This <mark style="color:red;">**is only partly true**</mark>. The real reason is we try to get to an expected noise at each sampling step. This is called the _<mark style="color:blue;">**noise schedule**</mark>_. Here is an example.

<figure><img src="../../.gitbook/assets/image (5).png" alt=""><figcaption><p>A noise schedule for 15 sampling steps.</p></figcaption></figure>

<mark style="color:green;">**The noise schedule is something we define**</mark>. We can choose to _<mark style="color:blue;">**subtract the same amount of noise at each step**</mark>_. <mark style="color:blue;">**Or**</mark> we can _<mark style="color:blue;">**subtract more in the beginning**</mark>_, <mark style="color:purple;">**like above**</mark>. _<mark style="color:orange;">**The sampler subtracts just enough noise in each step to reach the expected noise in the next step**</mark>_. That's what you see in the [step-by-step image](stable-diffusion-workflow.md#the-process-of-the-image-evolves-in-each-sampling-step).

## Image-to-image

Image-to-image is a method first proposed in the [SDEdit ](https://arxiv.org/abs/2108.01073)method. SDEdit can be applied to any diffusion model. So, we have image-to-image for Stable Diffusion (a latent diffusion model).

<mark style="color:red;">**An input image**</mark> and <mark style="color:red;">**a text prompt**</mark> _<mark style="color:blue;">**are supplied as the input**</mark>_ in image-to-image. The generated image will be conditioned by both the input image and text prompt. For example, using this amateur drawing and the prompt "photo of perfect green apple with stem, water droplets, dramatic lighting" as inputs, image-to-image can turn it into a professional drawing:

<figure><img src="../../.gitbook/assets/image (27).png" alt=""><figcaption><p>Image-to-image</p></figcaption></figure>

### Step1

The input image is encoded to latent space.

<figure><img src="../../.gitbook/assets/image (59).png" alt=""><figcaption></figcaption></figure>

### Step2

Noise is added to the latent image. [Denoising strength](denoising-strength.md) controls _<mark style="color:green;">**how much noise is added**</mark>_. If it is 0, no noise is added. _<mark style="color:red;">**If it is 1, the maximum amount of noise**</mark>_ is added so that _<mark style="color:red;">**the latent image becomes a complete random tensor**</mark>_.

<figure><img src="../../.gitbook/assets/image (3).png" alt=""><figcaption></figcaption></figure>

### Step3

The [_<mark style="color:green;">**noise predictor U-Net**</mark>_](diffusion-in-image.md#noise-predictor) _<mark style="color:green;">**takes the latent noisy image**</mark>_ and _<mark style="color:green;">**text prompt**</mark>_ _<mark style="color:red;">**as input**</mark>_ and _<mark style="color:red;">**predicts the noise in latent space (a 4x64x64 tensor)**</mark>_.

<figure><img src="../../.gitbook/assets/image (22).png" alt=""><figcaption></figcaption></figure>

### Step4

Subtract the latent noise from the latent image. This becomes our new latent image.

<figure><img src="../../.gitbook/assets/image (21).png" alt=""><figcaption></figcaption></figure>

Step3 3 and 4 are repeated for a certain number of sampling steps, for example, 20 times.

### Step5

Finally, the decoder of VAE converts the latent image back to pixel space. This is the image you get after running image-to-image.

<figure><img src="../../.gitbook/assets/image (11).png" alt=""><figcaption></figcaption></figure>

All _<mark style="color:blue;">**image-to-image does is to set the initial latent image**</mark>_ with _<mark style="color:red;">**a bit of noise**</mark>_ and _<mark style="color:red;">**a bit of input image**</mark>_. Setting _<mark style="color:green;">**denoising strength to 1 is equivalent to text-to-image**</mark>_ because the _<mark style="color:yellow;">**initial latent image is entirely random noise**</mark>_.

## Inpainting

Inpainting is really just <mark style="color:red;">**a particular case of image-to-image**</mark>. <mark style="color:blue;">**Noise is added to the parts of the image you wanted to impatient**</mark>**. **_<mark style="color:green;">**The amount of noise is similarly controlled by**</mark>_ [_<mark style="color:green;">**denoising strength**</mark>_](denoising-strength.md).

## Depth-to-image

[_<mark style="color:blue;">**Depth-to-image**</mark>_](depth-maps.md) _<mark style="color:blue;">**is an enhancement to image-to-image**</mark>_; <mark style="color:red;">**it generates new images with additional conditioning**</mark> using a [depth map](depth-maps.md).

### Step1

The input image is encoded into the latent state

<figure><img src="../../.gitbook/assets/image (26).png" alt=""><figcaption></figcaption></figure>

### Step2

[MiDAS ](https://github.com/isl-org/MiDaS)(an AI depth model) estimates the depth map from the input image.

<figure><img src="../../.gitbook/assets/image (25).png" alt=""><figcaption></figcaption></figure>

### Step3

Noise is added to the latent image. [Denoising strength](denoising-strength.md) controls how much noise is added. If the denoising strength is 0, no noise is added. If the denoising strength is 1, the maximum noise is added so that the latent image becomes a random tensor.

<figure><img src="../../.gitbook/assets/image (39).png" alt=""><figcaption></figcaption></figure>



### Step4

The noise predictor estimates the noise of the latent space, _<mark style="color:red;">**conditioned by the text prompt and the depth map**</mark>_.

<figure><img src="../../.gitbook/assets/image (29).png" alt=""><figcaption></figcaption></figure>

### Step5

Subtract the latent noise from the latent image. This becomes our <mark style="color:red;">**new latent image**</mark>.

<figure><img src="../../.gitbook/assets/image (17).png" alt=""><figcaption></figcaption></figure>

Steps 4 and 5 are repeated for the number of sampling steps.

### Step6

The decoder of VAE decodes the latent image. Now you get the final image from depth-to-image.

<figure><img src="../../.gitbook/assets/image (44).png" alt=""><figcaption></figcaption></figure>

## Credit

{% embed url="https://stable-diffusion-art.com/how-stable-diffusion-work/#Stable_Diffusion_step-by-step" %}
