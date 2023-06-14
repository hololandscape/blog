# 😄 ControlNet

## Overview

ControlNet is a neural network model for controlling [Stable Diffusion](../) models. You can use ControlNet _<mark style="color:red;">**along with**</mark>_ any Stable Diffusion models.

Stable diffusion models support text-to-image. It uses text prompts as the conditioning to steer image generation so that we generate images that match the text prompt.

ControlNet adds one more conditioning in addition to the text prompt. The extra conditioning can take many forms in ControlNet.

## How does ControlNet work?

ControlNet works by attaching trainable network modules to various parts of the U-Net (noise predictor) of the Stable Diffusion Model. The weight of the Stable Diffusion model is locked so that they are unchanged during training. Only the attached modules are modified during training.

<figure><img src="../../../.gitbook/assets/image (38).png" alt=""><figcaption></figcaption></figure>

According to the diagram from the [paper](https://arxiv.org/abs/2302.05543). Initially, the weights of the attached network module are all zero, making the new model able to take advantage of the trained and locked model.

During training, two conditionings are supplied along with each training image.

* The text prompt
* **The control map**
  * OpenPose
  * Canny edges
  * etc

The ControlNet model learns to generate images based on these two inputs.

Each control method is trained independently.

## Two ways to use ControlNet

* Edge detection
* Human pose detection

### Edge detection

ControlNet takes an additional input image and detects its outlines using the <mark style="color:red;">**Canny edge detector**</mark>. An image containing the detected edges is then saved as a <mark style="color:red;">**control map. It is fed into the  ControlNet model as an extra conditioning to the text prompt.**</mark>

<figure><img src="../../../.gitbook/assets/image (18).png" alt=""><figcaption><p>Stable Diffusion ControlNet with Canny edge conditioning. Source: stable diffusion art</p></figcaption></figure>

The process of extracting specific information(edge in this case) from the input image is called <mark style="color:red;">**annotation**</mark>([Adding Conditional Control to Text-to-image Diffusion Models](https://arxiv.org/abs/2302.05543)) or <mark style="color:red;">**preprocessing**</mark> (in the ControlNet extension).

### Human pose detection

[Openpose](https://github.com/CMU-Perceptual-Computing-Lab/openpose)(_<mark style="color:red;">**Edge detection is not the only way an image can be preprocessed**</mark>_) is a fast human keypoint detection model that can extract human poses like positions of hands, legs, and head, like:

<figure><img src="../../../.gitbook/assets/image (21).png" alt="" width="300"><figcaption><p>Input image annotated with human pose detection using Openpose.</p></figcaption></figure>

#### ControlNet workflow using OpenPose

<figure><img src="../../../.gitbook/assets/image (50).png" alt=""><figcaption></figcaption></figure>

In this ControlNet workflow, <mark style="color:purple;">**key points are extracted from the input image using OpenPose**</mark> and saved as a control map containing the positions of key points. It is then fed to Stable Diffusion as an <mark style="color:purple;">**extra conditioning**</mark> together with the text prompt. Images are generated based on these two conditionings.

### The difference between using Canny edge detection and Openpose

The <mark style="color:green;">**Canny edge detector**</mark> <mark style="color:red;">**extracts**</mark> the _<mark style="color:red;">**edges of the subject**</mark>_ and _<mark style="color:red;">**background alike**</mark>_. It tends to translate the scene _<mark style="color:red;">**more faithfully.**</mark>_ For instance, the outline and hairstyle are preserved in the pictures(edge detection) above.

The OpenPose(It reminds me of Xbox Kinect) only detects human key points such as positions of the head, arms, etc. The image generation is _<mark style="color:red;">**more liberal**</mark>_ but follows the original pose. For example, the woman jumping up with the left foot pointing sideways is <mark style="color:red;">**different from the original image and the one in the Canny Edge example**</mark> <mark style="color:purple;">**because**</mark> Openpose's keypoint detection does not specify the orientations of the feet.

## Difference between the Stable Diffusion depth model and ControlNet

Stability AI, the creator of Stable Diffusion, released a depth-to-image model. It shares a lot of similarities with ControlNet, but there are important differences.

### Similar

* Both are Stable Diffusion models
* Both use two conditionings (a preprocessed image and text prompt)
* Both use MiDAS to estimate the depth map

### Difference

* Depth-to-image model is a v2 model. ControlNet can be used with any v1 or v2 models. ControlNet can use **any** v1 model not only opening up depth conditioning to the v1.5 base model, but also thousands of special models that were released by the community.
* ControlNet is more versatile like condition with edge detection, pose detection, and so on.
* ControlNet's depth map has a higher resolution than depth-to-image's

## How does Con

## Credit

{% embed url="https://stable-diffusion-art.com/controlnet/#What_is_ControlNet" %}

