# 📙 LoRA(Stable Diffusion)

## Overview

[LoRA(Low-Rank Adaptation)](../lora/) models are small Stable Diffusion models that apply tiny changes to standard checkpoint models. They are usually 10 to 100 times smaller than checkpoint models. That makes them very attractive to people having an extensive collection of models.

## Why we need it?

It offers a good trade-off between file size and training power. For example, Dreambooth is powerful but results in large model files (2-7 GBs). [Textual inversions](embeddings.md) are tiny (about 100 KBs), but you can't do as much.

LoRA sits in between: Their files sizes are manageable (2-200 MBs), and the training power is decent.

_<mark style="color:red;">**LoRA is an excellent solution to the storage problem. Like textual inversion, you cannot use a LoRA model alone. It must be used with a model checkpoint file. LoRA modifiles styles by applying small changes to the accompanying model file.**</mark>_

## How doe LoRA work on Stable Diffusion models?

LoRA applies small changes to the most critical part of Stable Diffusion models:

### The cross attention layers

It is the part of the model where the [image and the prompt meet](conditioning.md#feeding-embeddings-to-noise-predictor). According to the pager, it sufficient to fine-tune this part of the model to achieve good training. The [cross-attention layers ](https://aisuko.gitbook.io/wiki/ai-techniques/stable-diffusion/conditioning#cross-attention)are the yellow parts in the Stable Diffusion architecture below.

<figure><img src="../../.gitbook/assets/image (40).png" alt=""><figcaption><p>LORA fine-tunes the cross-attention layers (the QKV parts of the U-Net noise predictor). (Figure from <a href="https://arxiv.org/abs/2112.10752">Stable Diffusion paper</a>.)</p></figcaption></figure>

The weights of a cross-attention layer are arranged in **matrices. A LoRA model fine-tunes a model by adding its weights to these matrices.**

## Tips

### LoRA model needs the following phrase in the prompt

\<lora:filename:multiplier>

#### filename

It is the file name of the LoRA model, excluding the extension (.pt, .bin.etc)

#### multiplier

It is the weight applied to the LoRA model. The defaults is 1. Setting it to 0 disables the model.

### How to make sure the filename is correct?

Here we can use the model button

<figure><img src="../../.gitbook/assets/image (46).png" alt=""><figcaption><p>Source: Stable Diffusion Art</p></figcaption></figure>



Click on the Lora tab. You should see a list of LoRA models installed. Click on the one you want to use. And the LoRA phrase will be inserted in the prompt.

<figure><img src="../../.gitbook/assets/image (11).png" alt=""><figcaption><p>Source: Stable Diffusion Art</p></figcaption></figure>

### Adjust the multiplier

You may **adjust the multiplier** to crank up or tune down the effect. Setting the multiplier 0 disables the LoRA model. You can adjust the style effect between 0 and 1.

### Adjust for Dreambooth

Some LoRA models are trained with Dreambooth. You will need to include a **trigger keyword** to use the LoRA model. You can find the trigger keyword on the model’s page.

### Using multiple models at same time

Similar to [embeddings](embeddings.md), you can use **multiple LoRA models** at the same time. You can also use them with embeddings.

### Add the LoRA to the prompt

In AUTOMATIC1111, the LoRA phrase is not part of the prompt. It will be removed after the LoRA model is applied. That means you cannot use [prompt syntax](../langchain/prompts.md) like \[keyword1:keyword2: 0.8] with them.

## Credit

{% embed url="https://stable-diffusion-art.com/lora/" %}
