# 📼 Stable Diffusion v1 vs v2

## Model difference

Stable Diffusion v2 uses [OpenClip ](https://stability.ai/blog/stable-diffusion-v2-release)for text embedding. Stable Diffusion v1 uses Open AI's [CLIP Vit-L/14](https://github.com/CompVis/stable-diffusion) for text embedding. The reasons for this change are:

### OpenClip is up five times larger

A larger text encoder model improves image quality

### More transparency

Open AI's CLIP models are opensource, but the models were trained with proprietary data.

## Training data difference

Stable Diffusion v1.4 is [trained ](https://huggingface.co/CompVis)with

* 237k steps at resolution 256×256 on [laion2B-en](https://huggingface.co/datasets/laion/laion2B-en) dataset.
* 194k steps at resolution 512×512 on [laion-high-resolution](https://huggingface.co/datasets/laion/laion-high-resolution).
* 225k steps at 512×512 on “[laion-aesthetics v2 5+](https://laion.ai/blog/laion-aesthetics/)“\
  with 10% dropping of text conditioning.

Stable Diffusion v2 is [trained](https://huggingface.co/stabilityai/stable-diffusion-2-base) with

* 550k steps at the resolution `256x256` on a subset of [LAION-5B](https://laion.ai/blog/laion-5b/) filtered for explicit pornographic material, using the [LAION-NSFW classifier](https://github.com/LAION-AI/CLIP-based-NSFW-Detector) with `punsafe=0.1` and an [aesthetic score](https://github.com/christophschuhmann/improved-aesthetic-predictor) >= `4.5`.&#x20;
* 850k steps at the resolution `512x512` on the same dataset on images with resolution `>= 512x512`.
* 150k steps using a [v-objective](https://arxiv.org/abs/2202.00512) on the same dataset.
* Resumed for another 140k steps on `768x768` images.

[Stable Diffusion v2.1](https://huggingface.co/stabilityai/stable-diffusion-2-1) is fine-tuned on v2.0

* additional 55k steps on the same dataset (with `punsafe=0.1`)
* another 155k extra steps with `punsafe=0.98`

So basically, they **turned off the NSFW filter in the last training steps.**

## Outcome Difference

_<mark style="color:red;">**Users generally find it harder to use Stable Diffusion v2 to control styles and generate celebrities.**</mark>_** Although Stability AI did not explicitly filter out artist and celebrity names, their effects are much weaker in v2**. This is likely due to the difference in training data. <mark style="color:red;">**Open AI’s proprietary data may have more artwork and celebrity photos**</mark>. Their data is <mark style="color:red;">**probably highly filtered so that everything and everyone looks fine and pretty.**</mark>

