---
description: Fine-tuning models on consumer hardware
---

# 🛻 Training with QLoRA

{% hint style="info" %}
Thanks for Benjamin Marie's [article](https://towardsdatascience.com/qlora-fine-tune-a-large-language-model-on-your-gpu-27bed5a03e2b), all the contents are from his article.
{% endhint %}

## Overview

I am interested in almost everything about AI with consumer hardware. So, let's look at this article.

It introduces the _<mark style="color:green;">**QLoRa**</mark>_ technique,

<figure><img src="../../.gitbook/assets/image (62).png" alt=""><figcaption><p>Source from the <a href="https://towardsdatascience.com/qlora-fine-tune-a-large-language-model-on-your-gpu-27bed5a03e2b">article</a></p></figcaption></figure>

Here are some data from the article. If we want to fine-tune a 65 billion parameters model we need more than 780 Gb of GPU memory. This is equivalent to ten A100 800 Gb GPUs.

And now, with [QLoRa](https://arxiv.org/pdf/2305.14314.pdf), we can do this with only one A100. And the computing resource in that article is an Nvidia RTX 3060 12Gb.(I do not think that the Google Colab has a free instance). And the model we use in the example is a GPT model with 20 billion parameters.

## What is QLoRa?

Quantized LLMs with [Low-Rank Adapters](https://arxiv.org/abs/2106.09685). It mentioned that "LoRa adds a tiny amount of trainable parameters, i.e., adapters, for each layer of the LLM and freezes all the original parameters. For fine-tuning, we only have to update the adapter weights which significantly reduces the memory footprint".

QLoRa goes three steps further by introducing:

### 4-bit NormalFloat quantization

This is a method that improves upon quantile quantization. It ensures an equal number of values in each quantization bin. This avoids computational issues and errors for outlier values.

### Double quantization

The authors of QLoRa define it as follows: "the process of quantizing the quantization constants for additional memory savings."

### Paging with unified memory

It relies on the NVIDIA Unified Memory feature and automatically handles page-to-page transfers between the CPU and GPU. It ensures error-free GPU processing, especially in situations where the GPU may run out of memory.

## Fine-tuning a GPT model with QLoRa

### Hardware requirements for QLoRa

#### GPU

We mentioned it above

#### RAM

At least 6 Gb

#### Hard drive

GPT-J and GPT-NEoX-20b are both very big models. I recommend at least 80 Gb of free space.

### Software requirements for QLoRa

We need CUDA and other dependencies:

#### bitsandbytes

A library that contains all we need to quantize an LLM

#### Hugging Face Transformers and Accelerate

These are standard libraries that are used to efficiently train models from Hugging Face hub.

#### PEFT

A library that provides the implementations for various methods to only fine-tune a small number of (extra) model parameters. It supports LoRa.

#### Datasets

This one is not a requirement. We will only use it to get a dataset for fine-tuning. Of course, you can provide instead your own dataset.

#### Get all of them

```
pip install -q -U bitsandbytes
pip install -q -U git+https://github.com/huggingface/transformers.git 
pip install -q -U git+https://github.com/huggingface/peft.git
pip install -q -U git+https://github.com/huggingface/accelerate.git
pip install -q datasets
```

## The Python script

### Loading and Quantization of a GPT model

```python
import torch
from transformers import AutoTokenizer, AutoModelForCausalLM, BitsAndBytesConfig
```

Here we will fine-tune the [GPT NeoX](https://huggingface.co/EleutherAI/gpt-neox-20b) model pre-trained by [EleutherAI](https://www.eleuther.ai/). This is <mark style="color:red;">**a model with 20 billion parameters**</mark>. Note: GPT NeoX has a permissive license (Apache 2.0) that **allows commercial use.**

#### Get the model and the associated tokenizer

```python
model_name ="EleutherAI/gpt-neox-20b"

#Tokenizer
toeknizer = AutoTokenizer.from_pretrained(model_name)
```

Then, we need to detail the configuration of the quantizer, as follows:

```python
quant_config=BitsAndBytesConfig(
    load_in_4bit=True,
    bnb_4bit_use_double_quant=True,
    bnb_4bit_quant_type="nf4",
    bnb_4bit_compute_dtype=torch.bfloat16
)
```

* <mark style="color:red;">**load\_in\_4bit**</mark>: The modeal will loaded in the memory with 4-bit precision
* <mark style="color:red;">**bnb\_4bit\_use\_double\_quant**</mark>: We will do the double quantization proposed by QLoRa.
* <mark style="color:red;">**bnb\_4bit-quant\_type**</mark>: This is the type of quantization. "nf4" stands for 4-bit NormalFloat.
* <mark style="color:red;">**bnb\_4bit\_compute\_dtype**</mark>: While we load the store the model in 4-bit, we will partially dequantize it when needed and do all the computations with 16-bit orecision (bfloat16).

#### Loading the model in 4-bit

```
model = AutoModelForCausalLM.from_pretrained(model_name, quantization_cinfig=quant_config, device_mapp={"":0})
```

Then, we enable gradient checkpointing:

```python
model.gradient_checkpointing_enable()
```

### Preprocessing the GPT model for LoRa

We use PEFT, we prepare the model for LoRa, adding trainable adapters for each layer.

```python
from peft import prepare_model_for_kbit_training, LoraConfig, get_peft_model

model=prepare_model_for_kbit_training(model)

config=LoraConfig(r=8,
                lora_alpha=32,
                targe_modules=["query_key_value"],
                lora_dropout=0.05,
                bias="none",
                task_type="CAUSAL_LM"
)

model=get_peft_model(model, config)
```

In LoraConfig, you can play with r, alpha, and dropout to obtain better results on your task. More details and options in the [PEFT repo](https://github.com/huggingface/peft/tree/main).

With LoRa, we add 8 million parameters. We will only train these parameters and freeze everthing else. Fine-tuning should be fast.

### Dataset

Here the author uses the "english\_quotes" dataset. This is a dataset made of famous quotes distrabuted under a CC BY 4.0 license.

```python
from datasets import load_dataset
data = load_dataset("Abirate/english_quotes")
data = data.map(lambda samples: tokenizer(samples["quote]), betched=True)
```

### Fine-tuning GPT-NeoX-20B with QLoRa

Fianlly, the fine-tuning with Hugging Face Transformers is very standard.

```python
import transformers

tokenizer.pad_token = tokenizer.eos_token

trainer = transformers.Trainer(
    model=model,
    train_dataset=data["train"],
    args=transformers.TrainingArguments(
        per_device_train_batch_size=1,
        gradient_accumulation_steps=8,
        warmup_steps=2,
        max_steps=20,
        learning_rate=2e-4,
        fp16=True,
        logging_steps=1,
        output_dir="outputs",
        optim="paged_adamw_8bit"
    ),
    data_collator=transformers.DataCollatorForLanguageModeling(tokenizer, mlm=False),
)
trainer.train()
```

Don't forget optim="paged\_adamw\_8bit". It activates the paging for better memory management. Without it, we ge out-of-memory errors.

The VRAM consumption should peak at 15Gb.

## GPT Inference with QLoRa

The QLoRa model we fine-tuned can be directly used with the standard Hugging Face Transformer'sinference, as follows:

```
text = "Ask not what your country"
device="cuda:0"
inputs=tokenizer(text, return_tensors="pt").to(device)

outputs = model.generate(**inputs, max_new_tokens=20)
print(tokenizer.decode(outputs[0],skip_special_tokens=True))
```

You should get this quote as output:

```
Ask not what your country can do for you, ask what you can do for your country.”

– John F.
```

## Conclusion

Thanks for the new techniques like LoRa, QLoRa, PEFT and DeepSpeed. We can fine-tune models with billion parameters without relying on cloud computing and without a significant drop in performance according to papers of these techniques.

## Credit

{% embed url="https://towardsdatascience.com/qlora-fine-tune-a-large-language-model-on-your-gpu-27bed5a03e2b" %}
