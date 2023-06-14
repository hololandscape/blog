---
description: Explaining the ControlNet settings
---

# 🪡 Settings Explained

## Input controls

### Image Canvas

It is used to drag/drop the input image here. You can also click on the canvas and select a file using the file browser. <mark style="color:red;">**The input image will be processed by the selected preprocessor in the Preprocessor dropdown menu. A control map will be created.**</mark>

### Write icon

Create a new canvas with a white image instead of uploading a reference image. It is for creating a scribble directly.

### Camera icon

Take a picture using your device's camera and use it as the input image. You will need to grant permission to your browser to access the camera.

## Model selection

### Enable

Whether to enable ControlNet

### Low VRAM

For GPU with less than 8GB VRAM. It is an experimental feature. Check if you are out of GPU memory or want to increase the number of images processed.

### Allow Preview

Check this to enable a preview window next to the reference image. Use the explosion icon next to the Preprocessor dropdown menu to preview the effect of the preprocessor.

### Preprocessor

The preprocessor for preprocessing the input image, such as detecting edges, depth, and normal maps. None uses the input image as the control map.

### Model

ControlNet model to use. If you have selected a preprocessor, you would normally select the corresponding model. The ControlNet model is used together with the Stable Diffusion model selected at the top of AUTOMATIC1111 GUI.

### Control Weight

{% hint style="info" %}
Weight: How much emphasis to give the control map relative to the prompt. It is similar to keyword weight in the prompt but applies to the control map.
{% endhint %}

It is used to control how much the control map is followed relative to the prompt. The lower the weight, the less ControlNet demands the image to follow the control map.

### Starting ControlNet step

The step ControlNet first applies. 0 means the very first step.

### Ending ControlNet step

The step ControlNet ends. 1 means the last step.

<mark style="color:red;">**The initial steps set the global composition**</mark>(_<mark style="color:green;">**The sampler removes the maximum amount of noise in each step, and it starts with a random tensor in latent space**</mark>_), the pose is set even if you only apply ControlNet to as few as 20% of the first sampling steps.

In conclusion, changing the ending ControlNet step has a smaller effect because the global composition is set in the beginning steps.

## Control Mode

### Balanced

The ControlNet is applied to both conditioning and unconditioning in a **sampling step**. This is the standard mode of operation.

### My prompt is more important

The effect of ControlNet is gradually reducing over the instances of U-Net injection (_<mark style="color:red;">**There are 13 of them in one sampling step**</mark>_). The net effect is your prompt has more influence than the ControlNet.

### ControlNet is more important

Turn off ControlNet on unconditioning. Effectively, the CFG scale also acts as a multiplier for the effect of the ControlNet.

## Resize mode

Resize mode controls what to do when the size of the input image or control map is different from the size of the images to be generated.

### Crop and Resize

Fits the image canvas to be within the control map. Crop the control map so that it is the same size as the canvas.

### Resize and fill

Fit the whole control map to the image canvas. Extend the control map with empty values so that it is the same size as the image canvas.

## Multiple ControlNets

It supports to use ControlNets multiple times to generate an image. (It helps us to pick different features from different pictures like CN0 provides pose, CN2 provides background, etc...)

{% hint style="info" %}
* Adjust ControlNet weights if one of them does not do its job.
* Pay attention to the resize mode if you have reference images of different sizes of the final image.
{% endhint %}

## Credit

{% embed url="https://stable-diffusion-art.com/controlnet/#Canny" %}
