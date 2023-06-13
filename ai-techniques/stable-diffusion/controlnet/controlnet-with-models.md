# 🐳 ControlNet with models

## Using Preview with the ControlNet plugin

The first step is to choose a preprocessor and it is helpful to turn on the <mark style="color:red;">**preview**</mark> so we know what the preprocessor is doing. Once the preprocessing is done, the original image is discarded, and only the preprocessed image will be used for ControlNet.

{% hint style="info" %}
Reduce the Control Weight will help with color issues or other artifacts
{% endhint %}

### Preprocessor and models

Once we choose a preprocessor, we must pick the correct model. <mark style="color:red;">**We need to do is select the model with the same starting keyword as the preprocessor.**</mark>

### OpenPose preprocessors

* **OpenPose**: eyes, noses, neck, shoulder, elbow, wrist, knees, and ankles
* **OpenPose\_face**: Openpose +facial detail
* **OpenPose\_hand**:OpenPose+hands and fingers
* **OpenPose\_faceonly**: facial details only
* **OpenPose\_full**: All of the above

### Tile resample model

The _<mark style="color:red;">**Tile**</mark>_ resample model is used for adding details to an image. It is often used [<mark style="color:red;">**with an upscaler**</mark>](https://github.com/Coyote-A/ultimate-upscale-for-automatic1111) to enlarge an image simultaneously.

### Reference preprocessors

_<mark style="color:blue;">**Reference is a new set of preprocessors**</mark>_ that let us _<mark style="color:purple;">**generate images similar to the reference image**</mark>_. The Stable Diffusion model and the prompt will still influence the images.

<mark style="color:red;">**Reference preprocessors do not use a control model. We only need to select the preprocessor but not the model. And there are 3 reference preprocessors:**</mark>

* Reference adain
  * Style transfer via [Adaptive Instance Normalization](https://arxiv.org/abs/1703.06868)
* Reference only
  * Link the reference image directly to the attention layers
* Reference adain+attn
  * Combination of above

### Canny edge detector(preprocessor+model)

[Canny edge detector](https://en.wikipedia.org/wiki/Canny\_edge\_detector) is a general-purpose, old-school edge detector.

It extracts the outlines of an image. It is useful for retaining the composition of the original image.

### Depth preprocessor

The depth preprocessor guesses the depth information from the reference image.

* **Depth Midas**
  * A classic depth estimator
* **Depth Leres**
  * More details but also tend to render background
* **Depth Leres++**
  * Even more details
* **Zoe**:
  * The level of detail sits between Midas and Leres.

### Line Art preprocessors

Line Art renders the outline of an image. It attempts to convert it to a sample drawing. The are a few line art preprocessors

* **Line art anime**: Anime-style lines
* **Line art anime denoise**: Anime-style lines with fewer details
* **Line art realistic**: Realistic-style lines
* **Line art coarse**: Realistic-style lines with heavier weight

### ControlNet Inpainting

ControlNet inpainting lets you use high denoising strength in inpainting to generate large variations without sacrificing consistency with the picture.

## Credit

{% embed url="https://stable-diffusion-art.com/controlnet/#Canny" %}
