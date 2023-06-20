# 🤼♀ The important parameters for stunning AI image

## Overview

Here is a primer for the basic generation parameters.

## CFG Scale

[<mark style="color:red;">**Classifier Free Guidance scale**</mark>](classifier-free-guidance.md) is a parameter to control how much the model should respect your prompt.

1-Mostly ignore your prompt

3-Be more creative

7-A good balance between following the prompt and freedom

15-Adhere more o prompt

30-Strictly follow the prompt

Below are a few examples of increasing the CFG scale with the same random seed. In general, you should stay away from the two extremes - 1 and 30.

<mark style="color:red;">**Recommendation**</mark>: Starts with 7. (Increase if you want it to follow your prompt more)

<figure><img src="../../.gitbook/assets/image (30).png" alt=""><figcaption><p>Higher CFG scale adheres more to the prompt.</p></figcaption></figure>

## Sampling steps

<mark style="color:red;">**Quality improves as the sampling step increases.**</mark>** **_<mark style="color:orange;">**Typically**</mark>, <mark style="color:orange;">**20 steps**</mark> with Euler sampler are <mark style="color:orange;">**enough**</mark> to reach a high quality, sharp image. **Although the image will still change subtly when stepping through to higher values,**** **<mark style="color:red;">**it will become different but not necessarily higher quality.**</mark>_

_<mark style="color:red;">**Recommendation: 20**</mark>_

<figure><img src="../../.gitbook/assets/image (47).png" alt=""><figcaption><p>Increasing sampling steps.</p></figcaption></figure>

## Sampling methods

<figure><img src="../../.gitbook/assets/image (46).png" alt=""><figcaption></figcaption></figure>

There is a variety of sampling methods you can choose, depending on what GUI you are using. They are different methods for solving diffusion equations. They are supposed to give the same result but could be slightly different due numerical bias. The only criteria is the image looks good, _<mark style="color:red;">**accuracy of the method should not be your concern.**</mark>_

Not all methods are created equal. Below are the processing time of various methods.

<figure><img src="../../.gitbook/assets/image (13).png" alt=""><figcaption><p>Rendering time for 20 steps.</p></figcaption></figure>

_<mark style="color:red;">**Recommendation: Euler**</mark>_

## Seed

<figure><img src="../../.gitbook/assets/image (35).png" alt=""><figcaption><p>Seed dialog box</p></figcaption></figure>

The random seed _<mark style="color:blue;">**determines the initial noise pattern and hence the final image.**</mark>_

_<mark style="color:red;">**Setting it to -1 means using a random one every time.**</mark>_ It is useful when you want to generate new images. On the other hand, fixing it would result in the same images in each new generation.

_<mark style="color:red;">**Recommendation: Set to -1 to explore. Fix to a value for fine-tuning.**</mark>_

## Image size

The size of output image. _<mark style="color:red;">**Since Stable Diffusion is trained with 512×512 images \[Warning: may out of date]**</mark>_, setting it to portrait or landscape sizes can create unexpected issues. Leave it as square whenever possible.

_<mark style="color:red;">**Recommendation: Set image size as 512×512.**</mark>_

## Batch size

_<mark style="color:red;">**Batch size is the number of images generated each time.**</mark>_ Since the <mark style="color:orange;">**final images are very dependent on the random seed**</mark>, it is always a good idea to generate a few images at a time. This way, you can get a good sense of what the current prompt can do.

_<mark style="color:red;">**Recommendation: Set batch size to 4 or 8.**</mark>_

## Restore faces

<figure><img src="../../.gitbook/assets/image (43).png" alt=""><figcaption></figcaption></figure>

Stable Diffusion is that it often has issues with faces and eyes. <mark style="color:red;">**Restore faces**</mark> is <mark style="color:red;">**a post-processing method**</mark> applied to images using AI trained specifically to <mark style="color:red;">**correct faces.**</mark>

<figure><img src="../../.gitbook/assets/image (60).png" alt=""><figcaption></figcaption></figure>

_<mark style="color:red;">**Recommendation: Turn restore faces on when you generate images with faces.**</mark>_

## Credit

{% embed url="https://stable-diffusion-art.com/know-these-important-parameters-for-stunning-ai-images/" %}
