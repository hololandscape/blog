# tokio

In Rust, we call package  as `Crate`. Crate tokio is a runtime for wrtiting reliable netwrok applications without compromising speed. And it is a event-driven, non-blocking I/O platform for writing asynchronous applications with Rust. It provides a rich set of components for building servers and clients.


## Working with Tasks

Asynchronous program in Rust are based around lightweigh, non-blocking units of execution called tasks.


The `tokio::task` module provides important tools for working with tasks:
* The `spawn` function and `JoinHandle` type, for scheduling a new task on the Tokio runtime and awaiting the output of a spawned task, respectively.
* Functions for running blocking operatiions in an asynchronous task context.

The `tokio::task` module is present only when the "rt" feature flag is enabled.

The crate Tokio provides a runtime for executing asynchronous tasks. Most applications can use the `#[tokio::main]` macro to run their code on the tokio runtime. However, this macro provides only basic configuration options. More advanced APIs for configuring and managing runtimes are provided by the `tokio::runtime` module.

Using the runtime requires the "rt" or "rt-multi-thread" feature flags, to enable the current-thread single-threaded scheduler and the multi-thread scheduler, respectively.


## CPU-bound tasks and blocking code

Tokio is able to concurrently run many tasks on a few thread by repeatedly swapping the currently running task on each thread.  However, this kind of swapping can only happen at `.await` points, so code that spends a long time wihtout reaching an `.await` will prevent other tasks from running. To combat this, Tokio provides two kinds of threads: COre threads and blocking threads.

The core threads are where all asynchronous code runs, and Tokio will by default spawn one for each CPU core. You can use the env variable `TOKIO_WORKER_THREADS` to override the default value.

The blocking threads are spawned on demand, can be used to run blocking code that would otherwise block other tasks from running and are kept alive when not used for a certain amount of time which can be configured with `thread_keep_alive`. Since it is not possible for Tokio to swap out blocking tasks, like it can do with asynchronous code, the upper limit on the number of blocking threads is very large. These limits can be configured on the Builder.
