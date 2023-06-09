# 👨🌾 ARM NEON

## Overview

ARM NEON is an <mark style="color:red;">**advanced**</mark> [Single Instruction Multiple Data (SIMD)](smid.md) <mark style="color:red;">**architecture extension**</mark> for <mark style="color:blue;">**the Arm Cortex-A**</mark> and <mark style="color:blue;">**Arm Cortex-R series**</mark> of processors. It is an evolution of ARM SIMD instructions and is billed as a "media engine" within the ARM core.

## NEON

<mark style="color:red;">**NEON is the ARMv8 version of SIMD**</mark>, where a <mark style="color:purple;">**single operation**</mark> performs the <mark style="color:purple;">**same operation**</mark> on <mark style="color:purple;">**multiple data elements**</mark> <mark style="color:red;">**in parallel.**</mark>

NEON technology provides a set of instructions that can be used to <mark style="color:red;">**accelerate multimedia**</mark> and signal processing applications, such as:

* image
* video processing
* audio encoding/decoding
* speech recognition

The GNU Compiler Collection (GCC) provides built-in intrinsics for the ARM Advanced SIMD extension, which are available when the <mark style="color:red;">**-mfpu=neon**</mark> switch is used.



