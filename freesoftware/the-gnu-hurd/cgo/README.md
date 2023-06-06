# 🚴♂ cgo

### Overview

Go language provides a pseudo-package called `"C"` to interface with C libraries. This package allows the Go program to call C functions and use C data types. To use C functions in Go, we can use the `"import C"` statement to import the C functions and data types into the Go program.

The C functions and data types can be accessed using the dot notation, for example, `C.function_name()`. To interface with C libraries, we can use \`cgo\`, which is a tool that generates Go code that can interact with C code.

### Example for import statement

```go
/*
#include <stdlib.h>
*/
import "C"
```

`Cgo` recognizes this comment above. Any lines starting with `#cgo` followed a space character are removed; these become directives for `cgo`.&#x20;

<mark style="color:red;">**C is a "pseudo-package" which means it is a special name interpreted by cgo as a reference to C's namespace.**</mark>

The remaining lines are used as a header when compiling the C parts of the package. In this case, those lines are just a single `#include` statement, but they can be almost any C code. The `#cgo`<mark style="color:red;">**directives are used to provide flags for the compiler and linker when building the C parts of the package.**</mark>

### Activities of the Go tool

When the Go tool sees that one or more Go files use the special import "C", it will look for other non-Go files in the directory and compile them as part of the Go package.

* Any .c, .s, .S or .sx files will be compiled with the C compiler. Any .cc, .cpp, or .cxx files will be compiled with the C++ compiler.&#x20;
* Any .f, .F, .for or .f90 files will be compiled with the fortran compiler.&#x20;
* Any .h, .hh, .hpp, or .hxx files will not be compiled separately, but, if these header files are changed, the package (including its non-Go source files) will be recompiled.

{% hint style="info" %}
Note that changes to files in other directories do not cause the package to be recompiled, so <mark style="color:red;">**all non-Go source code**</mark> for the package <mark style="color:red;">**should be stored in the package directory,**</mark> not in subdirectories.
{% endhint %}

#### Priority import path

"#include \<foo/bar.h>" will always find the local version in preference to any other version.

#### Default activities of cgo

cgo tool is enabled by default, but it is disabled by default when cross-compiling as well as when the CC env variable is unset and the default C compiler(gcc or clang) cannot be found on the system PATH.

Override the default by setting the \`CGO\_ENABLED\` env variable

* 1 to enable
* 0 to disable

Setting CC\_FOR\_${GOOS}\_${GOARCH} (for example, CC\_FOR\_linux\_arm) environment variable for supporting Cross-compiling

#### Limitation

If the program uses any `//export` directives, then the C code in the comment may only include declarations (`extern int f();`), not definitions (`int f() { return 1; }`). You can use `//export`directives to make Go functions accessible to C code.

<mark style="color:red;">**As Go doesn't have support for C's union type in the general case**</mark>, C's union types are represented as a Go byte array with the same length.

<mark style="color:red;">**Go structs cannot embed fields with C types.**</mark>

<mark style="color:red;">**Go code cannot refer to zero-sized fields that occur at the end of non-empty C structs.**</mark>

<mark style="color:red;">**Cgo translates C types into equivalent unexported Go types**</mark>. Because the translations are unexported, <mark style="color:red;">**a Go package should not expose C types in its exported API**</mark>: a <mark style="color:orange;">**C type used in one Go package is different from the same C type used in another.**</mark>

Calling C function pointers is currently not supported

#### Strings and things

C doesn’t have an explicit string type. Strings in C are represented by a zero-terminated array of chars.

So, here are the conversion functions:

{% hint style="info" %}
These conversions make a copy of the string data
{% endhint %}

* C.CString
* C.GoString
* C.GoStringN

```go
package print

// #include <stdio.h>
// #include <stdlib.h>
import "C"
import "unsafe"

func Print(s string) {
    cs := C.CString(s)
    C.fputs(cs, (*C.FILE)(C.stdout))
    C.free(unsafe.Pointer(cs))
}
```

Memory allocations made by C code are not known to Go's memory manager. When you create a C string with `C.CString` (or any C memory allocation) you must remember to free the memory when you're done with it by calling `C.free.`

```c
func Print(s string) {
    cs := C.CString(s)
    defer C.free(unsafe.Pointer(cs))
    C.fputs(cs, (*C.FILE)(C.stdout))
}
```

### Building cgo packages

To build `cgo` packages, just use `go build` or `go install` as usual.

### Reference

{% embed url="https://www.swig.org/Doc3.0/Go.html" %}

{% embed url="https://go.dev/blog/cgo" %}
