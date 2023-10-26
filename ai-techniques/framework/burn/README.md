# burn

Thanks to [lu-zero](https://github.com/lu-zero) that give me a good idea to start the Rust backend for LocalAI project with [burn](https://github.com/burn-rs/burn). 

<figure><img src="https://hostux.social/system/media_attachments/files/111/298/615/762/383/366/original/611e91636a3b2aa2.png" width="1000"><figcaption><p>Source: Author</figcaption></figure>


## Which platforms we need to choice for gRPC server by using burn?

The default acceleration of backend of the LocalAI project can support:

- [x] CPU
    - [x] OpenBLAS
- [x] GPU
    - [x] CUDA/CuBLAS - Nvidia
    - [x] Hipplas - AMD
    - [x] ClBLAS - AMD/Intel
    - [x] Metal -  Apple Silicon


So, the requirement of in here is that we need to have a backend that can support CPU and GPU, and as more as support the acceleration of CPU or GPU.


### Supproted platforms of burn

According to the [Supported Platforms of burn](https://github.com/burn-rs/burn#supported-platforms). We can se  [burn-ndarray backend](https://github.com/burn-rs/burn/tree/main/burn-ndarray) does not support GPU. And the mainstream of AI framework is based on GPU. So, it is not a good idea as a default backend.


And Burn torch backend is based on [tch-rs](https://github.com/LaurentMazare/tch-rs) crate, which offers a Rust interface to the PyTorch C++ API. And it supports:
- [x] CPU
- [x] CUDA
- [x] MPS
- [x] Vulkan

As you can see that all the GPU acceleration was already support by LocalAI project.

Burn WGPU Backend is using the [wgpu](https://github.com/gfx-rs/wgpu), and it supports
- [x] Vulkan
- [x] Metal
- [x] DX11/12
- [x] OpenGL
- [x] WebGPU

So, here we can see that the WGPU backend can support all the GPU acceleration that LocalAI project need except the CPU accelertion. However, I beleive WASM could have a feature. And more and more LLMs need GPU to get a better performance. Although we have Lora and QLora technologies to decrease the computing resources of using by LLMs. But, we still need to have a GPU acceleration for LLMs. And here are some issues about the WGPU backend:

* https://github.com/Gadersd/stable-diffusion-xl-burn/issues/2
* https://github.com/Gadersd/stable-diffusion-burn/issues/7
* https://github.com/Gadersd/stable-diffusion-burn/issues/5
* https://github.com/Gadersd/whisper-burn/issues/19


According to above, it would be a good choice that we choose the Burn torch backend as the default backend of LocalAI project.

Although we do not have a benchmark here to test the performance of the WGPU backend. But, we can implement it as first. And deal with in the future development cycles if we have a better backend. There are also some exmaples of using burn, please check the current project on [Github](https://github.com/hololandscape/blog/tree/main/ai-techniques/framework).

# Reference

* [Xmind address]([https://xmind.works/share/qMmOQDEx](https://xmind.works/share/qMmOQDEx))
* [Build of LocalAI](https://localai.io/basics/build/index.html)
