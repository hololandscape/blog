# 🚬 Classifier Free Guidance

## Overview

Classifier Free Guidance (CFG), a value AI artists tinker with every day.

## Classifier guidance

[Classifier guidance](https://arxiv.org/abs/2105.05233) is a way to <mark style="color:red;">**incorporate**</mark>** image labels** in diffusion models. You can use a label to guide the diffusion process. For example, the label "cat" steers the reverse diffusion process to generate photos of cats.

The <mark style="color:red;">**classifier guidance scale**</mark> is a parameter for controlling how closely the diffusion process should follow the label.

[Here is an example](https://arxiv.org/abs/2207.12598)r below. Suppose there are 3 groups of images with the label "cat", "dog" and "human". If _<mark style="color:red;">**the diffusion is unguided, the model will draw samples from each group's total population, but sometimes it may draw images that could fit two labels**</mark>_, e.g. a boy petting a dog.

<figure><img src="../../.gitbook/assets/image (59).png" alt=""><figcaption><p>Classifier guidance. Left: unguided. Middle: small guidance scale. Right: large guidance scale.</p></figcaption></figure>

<mark style="color:blue;">**With high classifier guidance**</mark>, the images produced by the diffusion model <mark style="color:purple;">**would be biased toward**</mark> the <mark style="color:red;">**extreme or unambiguous examples**</mark>. If you ask the model for a cat, it will return an image that is unambiguously a cat and nothing else.

The <mark style="color:green;">**classifier guidance scale**</mark> controls how closely the guidance is followed. In the figure above, _<mark style="color:red;">**the sampling on the right has a higher classifier guidance scale than the one in the middle**</mark>_. In practice, this scale value is simply the multiplier to the drift term toward the data with that label.

## Classifier-free guidance

Classifier guidance needs an extra model to provide that guidance, but this has presented some difficulties in training.

[Classifier-free guidance](https://arxiv.org/abs/2207.12598), in its authors' terms, is a way to achieve "classifier guidance without a classifier". _<mark style="color:yellow;">**Instead of using class labels and a separate model for guidance**</mark>_, they proposed to _<mark style="color:red;">**use image captions and train a conditional diffusion model**</mark>_, exactly like the [conditioning in text-to-image](conditioning.md#text-conditioning).

They put the <mark style="color:green;">**classifier part as conditioning of the noise predictor U-Net**</mark>, achieving the so-called "classifier-free" (i.e. _<mark style="color:blue;">**without a separate image classifier**</mark>_) guidance in image generation.

The text prompt provides this guidance in text-to-image.

### CGF value

How to control how much the guidance should be followed with classifier-free diffusion process via conditioning?

**Classifier-free guidance (CFG) scale is a value that controls how much the text prompt conditions the diffusion process.** The image generation is <mark style="color:red;">**unconditioned**</mark> (i.e. the prompt is ignored) when it is set to 0. A higher value steers the diffusion towards the prompt.

