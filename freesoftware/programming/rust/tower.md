# Crate tower

Tower is a library of modular and reusable components for building robust networking clients and servers.

Tower provides a simple core abstraction, the `Service` trait, which represents an asynchronous function taking a request and returning either a response or an error. This astraction can be used to model both client and servers.

Generic components, like `timeouts`, `rate limiting`, and `load balancing`, can be modeled as `Service`s that wrap some inner service and apply additional behavior before or after the inner service is called. This allows implementing these components in a protocol-agnostic, composable way. Typically, such services are referred to as middleware.

An additinal abstraction, the `Layer` trait, is used to compose middleware with `Service`s. If a `Service` can be thought of as an asynchronous function from a request type to a response type, a `Layer` is a functrion taking a `Service` of one typr and returning a `Service` of a different type. The `ServiceBuilder` type is used to add middleware to a service by composing it with multiple `Layer`s.
