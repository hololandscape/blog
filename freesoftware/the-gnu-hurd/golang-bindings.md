# 🚴♂ Golang Bindings

### Overview

Go language provides a pseudo-package called `"C"` to interface with C libraries. This package allows the Go program to call C functions and use C data types. To use C functions in Go, we can use the `"import C"` statement to import the C functions and data types into the Go program.

The C functions and data types can be accessed using the dot notation, for example, `C.function_name()`. To interface with C libraries, we can use `cgo`, which is a tool that generates Go code that can interact with C code.



### Example for import statement

```go
/*
#include <stdlib.h>
*/
import "C"
```

`Cgo` recognizes this comment above. Any lines starting with `#cgo` followed a space character are removed; these become directives for `cgo`.&#x20;

The remaining lines are used as a header when compiling the C parts of the package. In this case, those lines are just a single `#include` statement, but they can be almost any C code. The `#cgo`directives are used to provide flags for the compiler and linker when building the C parts of the package.

#### Limitation

if your program uses any `//export` directives, then the C code in the comment may only include declarations (`extern int f();`), not definitions (`int f() { return 1; }`). You can use `//export`directives to make Go functions accessible to C code.

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

###

### Repos

{% embed url="https://github.com/go-skynet/go-llama.cpp" %}
Go binding C
{% endembed %}

{% embed url="https://go.dev/blog/cgo" %}
