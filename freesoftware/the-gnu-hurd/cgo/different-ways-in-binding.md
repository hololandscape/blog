---
description: Using shared object binary file in go binding
---

# Different ways in binding

## Overview

The main difference between using cgo and SWIG to call C/C++ code from Go is that cgo generates wrappers to call C code from Go, while SWIG generates wrappers to call both C and C++ code from Go. Here are some more differences:

### cgo

* cgo is a tool provided by Go that enables the creation of Go packages that call C code.
* cgo generates wrappers to call C code from Go, but there is no convenient way to call C++ code from Go.
* cgo requires a C toolchain to be installed.
* cgo provides a pseudo-package "C" that can be used to import C code into Go code.
* cgo translates C types into equivalent unexported Go types, so a Go package should not expose C types in its exported API.
* cgo can be used to export C functions and types in a Go package's exported API using the `//export` comment.

### SWIG

* SWIG is a software development tool that generates wrappers to call both C and C++ code from Go.
* SWIG generates wrappers to call C/C++ code from Go, filling the gap left by cgo for calling C++ code from Go.
* SWIG supports both the gc compiler of the Go distribution and the gccgo compiler.
* SWIG generates wrapper code that can be compiled into a Go package.
* SWIG requires an interface file that describes the C/C++ code to be used in Go.
* SWIG can generate a natural Go interface to the C/C++ code, but some modifications may be required due to the differences between the languages.
* SWIG can be used to generate wrappers to call C/C++ code from Go on different platforms.

Note that both cgo and SWIG should be used with caution, as they can lead to compatibility issues between different Go packages that use the same C/C++ types. Additionally, using cgo or SWIG can result in a loss of type safety and cross-compilation targets that Go provides.

## Reference

[https://github.com/vladimirvivien/go-cshared-examples](https://github.com/vladimirvivien/go-cshared-examples)
