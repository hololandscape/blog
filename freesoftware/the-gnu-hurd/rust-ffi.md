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
* &#x20;constants

It saves you from manually writing the FFI declarations and ensures that the generated Rust code matches the C/C++ interface. It's commonly used when you want to create Rust bindings for a C/C++ library.

#### Summary

According to the above, `libc` helps with accessing C types and functions, while `bindgen` automates the process of generating Rust FFI bindings based on C/C++ headers.

### Repos

{% embed url="https://github.com/rust-lang/rust-bindgen" %}
bindgen
{% endembed %}

{% embed url="https://github.com/rust-lang/libc" %}
libc for Rust
{% endembed %}
