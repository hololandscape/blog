---
description: The textual inversion
---

# ⚕ Embeddings

## Overview

Embedding is the result of [textual inversion](https://textual-inversion.github.io/), a <mark style="color:red;">**method to define new keywords in a model without modifying it**</mark>. The method has gained attention because _<mark style="color:red;">**its capable of injecting new styles or objects to a model with as few as 3 -5 sample images**</mark>_.

## How does textual inversion work?

The textual inversion is NOT the ability to add new styles or objects — other fine-tuning methods can do that as well or better. <mark style="color:orange;">**It is the fact that it can do so**</mark><mark style="color:orange;">** **</mark>_<mark style="color:orange;">**without**</mark>_<mark style="color:orange;">** **</mark><mark style="color:orange;">**changing the model.**</mark>

<figure><img src="../../.gitbook/assets/image (36).png" alt=""><figcaption><p>New embedding is found for the new token S* through textual inversion. Source: original research article</p></figcaption></figure>

<mark style="color:red;">**First you define a new keyword that’s not in the model for the new object or style**</mark>. That _<mark style="color:red;">**new keyword will get tokenized**</mark>_ (that _<mark style="color:red;">**is represented by a number**</mark>_) just like any other keywords in the prompt.

Each token is then _<mark style="color:blue;">**converted to a unique embedding vector**</mark>_ to be used by the model for image generation.

_<mark style="color:green;">**Textual inversion finds the embedding vector of the new keyword**</mark>_ that best represents the new style or object, without changing any part of the model. You can think of it as finding a way _within_ the language model to describe the new concept.

## Example of embeddings

### Embedding an object

Toy cat can be used with other existing concepts (boat, backpack) in the model

<figure><img src="../../.gitbook/assets/image (6).png" alt=""><figcaption><p>Example of embedding an object.</p></figcaption></figure>

### Embedding a style

<figure><img src="../../.gitbook/assets/image (52).png" alt=""><figcaption><p>Example of embedding a style.</p></figcaption></figure>

## Where to find embeddings?

Hugging Face host the [Stable Diffusion Concept Library](https://huggingface.co/sd-concepts-library), which is a repository of large number of custom embeddings.

[**Civtai**](https://civitai.com/) is another great site you can browse models, including embeddings. Filter with **textual inversion** to view embeddings only.

## How to use embeddings?

### Web interface

[Stable Diffusion Conceptualizer ](https://huggingface.co/spaces/sd-concepts-library/stable-diffusion-conceptualizer)is a great way to try out embeddings without downloading them.

The downside of web interface is you cannot use the embedding with a different model or change any parameters.

### AUTOMATIC111

First, download an embedding file from the [Concept Library](https://huggingface.co/sd-concepts-library). It is the file named `learned_embedds.bin`. Make sure don’t right click and save in the below screen. That will save a webpage that it links to. Click of the file name and click the download button in the next page.

<figure><img src="../../.gitbook/assets/image (42).png" alt=""><figcaption></figcaption></figure>

Next, **rename the file as the keyword you wanted to use** this embedding with. It has to be something not exist in the model. `marc_allante.bin` is a good choice.

Put it in the `embeddings` folder in the GUI’s working directory: \`stable-diffusion-webui/embeddings\`

Restart the GUI. In startup terminal, you should see a message like:

```
Loaded a total of 1 textual inversion embeddings.
Embeddings: marc_allante
```

Use the filename as part of the prompt to

```
(marc_allante:1.2)  a dog
```

### Checking the embeddings are using in AUTOMATIC1111

There’s a button between the trash and the copy buttons:

<figure><img src="../../.gitbook/assets/image (48).png" alt=""><figcaption></figcaption></figure>

Click it and you will see all the embeddings that are available. They are all under the **Textual Inversion** tab.

<figure><img src="../../.gitbook/assets/image (37).png" alt=""><figcaption></figcaption></figure>

<mark style="color:red;">**Clicking any of them will insert that into the prompt. This function is especially useful to eliminate the tedious work of making sure you’ve entered the embedding magic word correctly.**</mark>

## Pros and cons of using embedding

### Pros

* It is small size (100KB or less)

### Cons

* The drawback of using embedding is sometimes its not clear which model it is supposed to be used with.

## Credit

{% embed url="https://stable-diffusion-art.com/embedding/" %}
