# Diffusers

## Overview
Huggingce Diffusers is the go-to library for state-of-the-art pretrained diffusion models for generating images, audio, and even 3D structures of molecules. It can be used to:
* The simple inference solution
* Training customize diffusion model

It focus on:
* Usability over performance
* Simple over easy
* Customizability over abstractions

## The main components
It has three main componenets:
* State-of-the-art `diffusion pipelines` for interence with code
* Interchangeable `noise schedulers` for balancing trade-offs between generation speed and quality
* Pretrained `models` that can be used as building blocks, and combined with schedulers, for creating end-to-end diffusion systems

## Type of the pipline(currently from diffusers)
Type|Description
---|---
img2img-text|image to image text guided generation
un-audio-gen|unconditional Audio generation
image|image generation
img2img|image to image generation
t-2-img|text to image
s-r-img2img|super resolution img2img
un-img-gen|unconditional image generation
img-g-img|image guided image inpainting




# Reference
[Huggingface diffusers](https://huggingface.co/docs/diffusers/index)