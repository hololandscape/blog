---
description: Rust Foreign Function Interface
---

# 🛃 Rust FFI

### Overview

Using Rust to create bindings for a C/C++ repository is definitely possible and can provide the benefits of Rust's memory safety and performance while leveraging existing C/C++ code.

### Proposal

Here is a high-level overview of the steps involved in creating bindings between Rust and a C/C++ repo:

1. Initial the Rust project
2. <mark style="color:purple;">**Define Rust FFI declarations**</mark>: Identify the C/C++ functions, types, and data structures you want to interact with from Rust. Declare their counterparts in Rust using the `extern` keyword and appropriate annotations to specify the C/C++ function signatures, data types, and any necessary safety guarantees.
3. <mark style="color:purple;">**Link the C/C++ code**</mark>: Specify the C/C++ codebase you want to bind to in your Rust project's build configuration. This involves specifying the necessary C/C++ headers and libraries for Rust to link against.
4. <mark style="color:purple;">**Write Rust wrapper code**</mark>: Create Rust code that wraps the FFI declarations. This wrapper code acts as a bridge between the Rust code and the underlying C/C++ functionality. It handles the conversion of data between the Rust and C/C++ representations, making the interaction seamless.
5. <mark style="color:purple;">**Build and test the Rust bindings**</mark>: Use Cargo to build the Rust project and ensure that the bindings are correctly generated. Write tests to verify the behavior and correctness of the Rust bindings, ensuring they match the expected behavior of the original C/C++ code.
6. <mark style="color:purple;">**Integrate the Rust bindings:**</mark> Once the Rust bindings are working as expected, you can integrate them into your Rust application or library. This allows you to utilize the functionality provided by the C/C++ code within your Rust codebase.

### Libraries

There are some libraries and frameworks available in the Rust ecosystem that can help in creating bindings, such as:

* `libc`
* `bindgen`

They can automate parts of the process by generating Rust FFI declarations based on C/C++ headers.

### Details for the different libraries

#### libc

It <mark style="color:purple;">**provides Rust bindings to the C standard library (libc).**</mark> It includes definitions for C types, constants, and functions that are commonly used in C programming.

It is useful when you need to interact with the C standard library or C APIs <mark style="color:purple;">**directly from Rust**</mark>. It simplifies the process of working with C types and provides a common interface for accessing C functionality.

#### bindgen

It is a Rust library that <mark style="color:orange;">**generates Rust FFI bindings by parsing C/C++ header files**</mark>.&#x20;

It analyzes the headers and <mark style="color:orange;">**automatically**</mark> generates Rust code that corresponds to the C/C++ declarations, including:

* struct definitions
* function signatures
* constants

It saves you from manually writing the FFI declarations and ensures that the generated Rust code matches the C/C++ interface. It's commonly used when you want to create Rust bindings for a C/C++ library.

#### Summary

According to the above, `libc` helps with accessing C types and functions, while `bindgen` automates the process of generating Rust FFI bindings based on C/C++ headers.

### Tutorials for Rust binding C

Using `C` or `C++` inside of a Rust project consists of two major parts:

* Wrapping the exposed C API for use with Rust
* Building your C or C++ code to be integrated with the Rust code

As C++ does not have a stable ABI for the Rust compiler to target, it is recommended to use the <mark style="color:red;">`C ABI`</mark> when combining Rust with C or C++.

### Defining the interface

It is necessary to define (in Rust) what data types and function signatures exist in the linked code. In C or C++, you would include a header(`.h` or `.hpp`) file which defines this data.

### Wrapping C functions and Datatypes

We will cover manually translating these definitions from C/C++ to Rust. Libraries written in C or C++ will provide a header file defining all types and functions used in public interfaces. It may looks like this:

```c
/* File: cool.h */
typedef struct CoolStruct {
    int x;
    int y;
} CoolStruct;

void cool_function(int i, char c, CoolStruct* cs);

```

When translated to Rust, this interface would look as such:

```rust
/* File: cool_bindings.rs */
#[repr(C)]
pub struct CoolStruct {
    pub x: cty::c_int,
    pub y: cty::c_int,
}

extern "C" {
    pub fn cool_function(
        i: cty::c_int,
        c: cty::c_char,
        cs: *mut CoolStruct
    );
}
```

By default, Rust does not guarantee order, padding, or the size of data included in a `struct`. In order to guarantee compatibility with the code, we use `#repr(C)]`attribute that instructs the Rust compiler to always use the same rules C does for organizing data with a struct.

```rust
#[repr(C)]
pub struct CoolStruct { ... }
```

Due to the flexibility of how C or C++ defines and `int` or `char` , it is recommended to use primitive data types defined in `cty`, which will map types from C to types in Rust.

```rust
pub x: cty::c_int,
pub y: cty::c_int,
```

The statement defines the signature of a function that uses the `C ABI` called `cool_function`

```rust
extern "C" { pub fn cool_function( ... ); }
```

Since C does not have a concept of Rust's references, which would like this `&mut CoolStruct` we instead have a raw pointer but this pointer is `unsafe`, and the pointer may in fact be a null pointer.&#x20;

### Automatically generating the interface

1. Gather all C or C++ headers defining interfaces or datatypes you would like to use with Rust.
2. Write a `bindings.h`  file, which `#include "..."` each of the files you gathered in step one.
3. Feed this `binding.sh` file, along with any compilation flash used to compile your code into `bindgen`.&#x20;
   1. &#x20;Tip: use `Builder.ctypes_prefix("cty")` / `--ctypes-prefix=cty` and `Builder.use_core()` / `--use-core` to make the generated code `#![no_std]` compatible.
4. `bindgen` will produce the generated Rust code to the output of the terminal window. This file may be piped to a file in your project, such as `bindings.rs`. You may use this file in your Rust project to interact with C/C++ code compiled and linked as an external library. Tip: don't forget to use the [`cty`](https://crates.io/crates/cty) crate if your types in the generated bindings are prefixed with `cty`.\


### Building your C/C++ code

As the Rust compiler does not directly know how to compile C or C++ code (or code from any other language, which presents a C interface), it is necessary to compile your non-Rust code ahead of time.

For embedded projects, this most commonly means <mark style="color:blue;">**compiling the C/C++ code to a static archive**</mark> (such as `cool-library.a`), which can then be combined with your Rust code at the final linking step.

Using already distributed as a static archive

* Just convert the provided interface header file as described above, and include the static archive at compile/link time.

Using code exists as a source project

* it will be necessary to compile your C/C++ code to a static library, either by triggering your existing build system (such as `make`, `CMake`, etc.), or by porting the necessary compilation steps to use a tool called the `cc` crate. For both of these steps, it is necessary to use a `build.rs` script.

### Rust build.rx build scripts

A `build.rs` script is a file written in Rust syntax, that is executed on your compilation machine, <mark style="color:blue;">AFTER dependencies of your project have been built</mark>, but <mark style="color:orange;">BEFORE your project is built.</mark>

&#x20;`build.rs` scripts are useful for generating code (such as via [bindgen](https://github.com/rust-lang/rust-bindgen)), calling out to external build systems such as `Make`, or directly compiling C/C++ through the use of the `cc` crate.

### Triggering external build systems

For projects with complex external projects or build systems, it may be easiest to use [`std::process::Command`](https://doc.rust-lang.org/std/process/struct.Command.html) to "shell out" to your other build systems by traversing relative paths, calling a fixed command (such as `make library`), and then copying the resulting static library to the proper location in the `target` build directory.

Targeting a `no_std` embedded platform means your `build.rs` executes only on machines compiling your crate.

#### Building C/C++ code with the `cc` crate <a href="#building-cc-code-with-the-cc-crate" id="building-cc-code-with-the-cc-crate"></a>

For projects with limited dependencies or complexity, it may be easier to instead utilize the cc crate, which provides an idiomatic Rust interface to the compiler provided by the host.

For example below, `cargo build` will compile and execute it before the build of the package. A static archive named `libfoo.a` is generated and placed in the `target` directory.

```rust
fn main() {
    cc::Build::new()
        .file("src/foo.c")
        .compile("foo");
}
```

### Repos

{% embed url="https://github.com/rust-lang/rust-bindgen" %}
bindgen
{% endembed %}

{% embed url="https://github.com/rust-lang/libc" %}
libc for Rust
{% endembed %}

{% embed url="https://github.com/rustformers/llm" %}
llm(Rust binding C)
{% endembed %}

{% embed url="https://docs.rust-embedded.org/book/interoperability/c-with-rust.html" %}
The Embedded Rust Book
{% endembed %}
