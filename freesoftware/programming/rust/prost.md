# Crate prost

`prost` is a Protocol Buffers implementation for the Rust language. `prost` generates simple, idiomatic Rust code from `proto2` and `proto3` files.

The features of `prost`:
* Generates simple, idiomatic, and readble Rust types by taking advantage of Rust `derive` attributes.
* Retains comments from `.proto` files in generated Rust code.
* Allows existing Rust types (not generated from a .proto) to be serialized and deseri8alized by adding attributes.
* Uses the `bytes::{Buf, BufMut}` abstractions for seialization instead of `std::io::{Read, Write}`
* Respects the Protobuf `package` specifier when organizing generated code into Rust modules.
* Preserves unknown enum values during dserialization
* Does not include support for runtime reflection or message descriptor