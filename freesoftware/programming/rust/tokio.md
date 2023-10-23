# Crate tokio

In Rust, we call package  as `Crate`. Crate tokio is a runtime for wrtiting reliable netwrok applications without compromising speed. And it is a event-driven, non-blocking I/O platform for writing asynchronous applications with Rust. It provides a rich set of components for building servers and clients.


## Tokio provides a few major components:
* A multi-thread runtime for executing components
* An asynchronous version of the standard library
* A large ecosystem of libraries


## Tokio's role in your project

Asynchronous Ruse code does not run on it owns, so you must choose a runtime to execute it. The Tokio library is the most widely used runtime, surpassing all other runtimes in usage combined.

And Tokio provides the asyncchronous versions of the standard library.


## When not to use Tokio

* Tojio is designed for IO-bound applications where each individual task spends most of its time waiting got IO. If the only thing your application does is run computations in parallel, you should be using rayon.

* Reading a lot of files.

* Sending a single web request.


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


## What is asynchronous programming?

Most computer programs are executed in the same order in which they are written. The first line executes, then the next, and so on. With synchronous programming, when a program encounters an operation that cannot be completed immediately, it will block until the operation completes. For example, establishing a TCP connection requires an exchange with a peer over the network, which can take a sizeable amount of time. During this time, the thread is blocked.

With asynchronous programming, operations that cannot complete immediately are suspended to the background. The thread is not blocked, and can continue running other things. Once the operation completes, the task is unsuspended and continues processing from where it left off.

Although asynchronous programming can result in faster applications, it often results in much more complicated programs. The programmer is required to track all the state necessary to resume work once the asynchronous operation completes. Historically, this is a tedious and error-prone task.


## Compile-time green-threading

Rust implements asynchronous programing using a feature called `async/await`. Functions that perform asynchronous operations are labeled with the `async` keyword. For example, the connect function is defined like this:

```
use mini_redis::Result;
use mini_redis::client::Client;
use tokio::net::ToSocketAddrs;

pub async fn connect<T: ToSocketAddrs>(addr: T) -> Result<Client> {
    // ...
}
```

The `async fn` definition looks like a regular synchronous function, but operate asynchronously. Rust transforms the `async fn` at compile time into a routine that operates asynchronously. Any calls to `.await` within the `async fn` yield control bakc to the thread. The thread may do other work while the operation processes in the background. 


# Reference
* [Tokio](https://tokio.rs/)