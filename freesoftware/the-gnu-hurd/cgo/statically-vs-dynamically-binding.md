---
description: >-
  What is the difference between statically binding and dynamically loading a
  shared library in Go
---

# Statically VS Dynamically binding

## Overview

Statically binding and dynamically loading a shared library in Go have the following differences:

### Statically Binding:

* The shared library is linked at compilation time.
* The linker and OS connect a library function to a memory address.
* The function is part of the program inside the executable as it was written by us for that program.
* The executable file size is larger because the library functions are included in the executable.
* The library functions are available immediately when the program starts.

### Dynamically Loading:&#x20;

* The shared library is linked at runtime.
* The function symbols are dynamically loaded and bound at runtime.
* The shared library is loaded into memory only when needed.
* The executable file size is smaller because the library functions are not included in the executable.
* The library functions are not available immediately when the program starts, but only when the shared library is loaded into memory.

In Go, there are <mark style="color:red;">**two ways to use a shared object library to call Go functions**</mark> from C:&#x20;

* Statically binding the shared library at compilation but dynamically linking it at runtime.
* Dynamically loading the shared library and binding the Go function symbols at runtime.
