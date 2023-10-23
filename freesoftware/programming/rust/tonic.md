# Crate tonic

A Rust implementation of gRPC, a high performance, open source, general RPC framework that puts mobile and HTTP/2 first. And it is a gRPC over HTTP/2 implementation focused on:

* **high performance**
* **interoperability**
* **flexibility**

And the library was created to have first class support of async/await and to act as a core building block for production systems wriiten in Rust.


# Features Flags

* `transport` Enable the fully featured, batteries included client and serber implementation based on the [`hyper`](./hyper.md), [`tower`](./tower.md) and [`tokio`](./tokio.md) crate. Enabled by default.
* `channel` Enables just the full featured channel/client portion of the `transport` feature
* `codegen` Enables all the required exports and optional dependencies required for `tonic-build`. Enabled by default.
* `tls` Enables the `rustls` based TLS options for the `transport` feature. Not enabled by default.
* `tls-roots` Adds system trust roots to `rustls` based gRPC client using the `rustls-native-certs` crate. Not enabled by default.
* `tls-webpki-roots` Add the standard trust roots from the `webpki-roots` crate to `rustls` based gRPC clients. Not enabled by default.
* [`prost`](./prost.md) Enables the prost based gRPC [Codec](./codec.md) implementation
* `gzip` Enables compressing requests, responses, and streams. Depends on flate2. Not enabled by default. Replaces the `compression` flag from earlier versions of tonic(<=0.7>).

# Usage

Tower provides an abstraction layer, and generic implementation of various middleware. This menas that the `tower` crate on its own does not provide a working implementation of a network client ot server. Instead, Tower's Service trait provides an integration point between application code, libraries providing middleware implementations, and libraries that implement servers and/or clients for various network protocols.

Depending on your particulat use case, you might use Tower in several ways:

* Implementing application logic for a networked program
* Implementing middleware to add custom behavior to network clients and servers in a reusable manner
* Implementing a network protocol
