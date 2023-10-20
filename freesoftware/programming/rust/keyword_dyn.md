# Keyword dyn

`dyn` is a prefix of a trait object type. The `dyn` keyword is used to highlight that calls to methods on the associated `Trait` are `dynamically dispatched`. To use the trait this way, it must be `object safe`.

For example below,

```rust
Box<dyn std::error:Error>
```

Here we create a dynamic trait object that can hold any type that implements the `std:error::Error` trait. This is possible because Rust's trait system is designed to support dynamic dispatch, which allows a function to call a method on a trait object without knowing the exact type of the obejct at complie time.

And this is also useful because it allows the caller of the funtion to handle the error in a generic way, without knowing the exact type of the error that occurred.

In summary, the `dyn` keyword is used to create a dynamic trait object that can hold any type that implements a given trait. This is useful for creating generic types and functions that can work with multiple types that implement the same trait.


# The related machanism

`Dynamic dispatch` in Rust is a mechanism that allows a function to call a method on a trait object without knowing the exact type of the obejct at compile time. This is useful when working with generic types or when the exact type of an object it not know until runtime.

## Static dispatch

In Rust, static dispatch is the default mechanism for calling methods on objects. With static dispatch, the compiler generates code that calls the method directly on the object's type. This is efficient and allows the compiler to optimize the code, but it requires the exact type of the obeject to be known at complie time.

## Dynamic dispatch

Dynamic dispatch, is used when the exact type of an object it not known until runtime. In this case, the compiler generates code that calls the method through a vtable, which is a table of function pointers that maps the method to its implementation at runtime. This allows the method to be called on the object without knowing its exact type at compile time.

And to use dynamic dispatch in Rust, you need to create a trait object by using the `dyn` keyword. A trait object is a pointer to an object that implements a trait, and it can be used to call methods on the object without knowing its exact type at compile time.

Here is an example:

```rust
trait Animal {
    fn speak(&self);
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

struct Cat;
impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];

    for animal in animals {
        animal.speak();
    }
}
```
