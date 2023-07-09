# Day-2 Afternoon

## Control flow

### Blocks

A block in Rust has a value and a type: the value is the last expression of the block

[snippet](./snippets/src/blocks.rs)

The same rule is used for functions
However, if the last expression end with `;`, then the resulting value and type is ()

### if expression

Unlike other languages, 'if' is an expression

### if let expression

If you want to match a value against a pattern, you can use `if let`

[snippet](./snippets/src/if_let.rs)

Unlike `match`, `if let does not support guard clauses for pattern matching`

### while loops

[snippet](./snippets/src/while_loop.rs)

### while let loops

Similar to `if let`, there is `while let` variant which **repeatedly** tests a value againnst a patter

[snippet](./snippets/src/while_let_loop.rs)

> the `while let loop` will keep going as long as the value matches the pattern.

### For loops

The `for loop` is closely related to the `while let loop`. It will automatically call into_iter() on the expression and the iterate over it:

[snippet](./snippets/src/for_loop.rs)

you can use `break` and `continue` here as usual.

### Loop expressions

There is a `loop keyword` which creates an endless loop.
Here, you must either `break` or `return` to stop the loop

[snippet](./snippets/src/loop_expr.rs)

### Match expressions

The `match keyword` is used to match a value against one or more patterns. In that sense, it works like a series of `if let` expressions:

[snippet](./snippets/src/match_expr.rs)

Like `if let`, each match arm must have the same type. The type is the last expression of the block, if any.

### break and continue

Both `continue` and `break` can optionally take a label argument which is used to break out of nested loops

[snippet](./snippets/src/break_and_continue.rs)

## Standard Library

Rust comes with a standard library which helps establish a set of common types used by Rust library and programs.

Common vocabulary types include:

- `Option and Result` types: used for optional values and **error handling**.
- `String`: the default string type used for owned data.
- `Vec`: a standard extensible vector.
- `HashMap`: a hash map type with a configurable hasing algorithm.
- `Box`: an owned pointer for heap-allocated data.
- `Rc`: a shared reference-counted pointer for heap-allocated data.

Rust layers of the Standard Library:

- `core` : most basic types and functions that don't depend on `libc`, allocator or even the presence of an operating system.
- `alloc`: includes types which require a global heap allocator, such as `Vec`, `Box`, and `Arc`.
- `std`

### Option and Result

The types represent optional data

[snippet](./snippets/src/option_and_result.rs)

> - `Option` and `Result` are widely used not just in the standard library.
> - `Option<&T>` has zero space overhead compared to `&T`.
> - `Result` is the standard type to implement error handling.

### String

`String` is the standard heap-allocated growable UTF-8 string buffer

[snippet](./snippets/src/rust_string.rs)

`String` implements `Deref<Target = str>` which transparently gives it access to `str`'s methods.

> - `String::new` returns a new empty string, use `String::with_capacity` when you know how much data you want to push to the string.
> - `String::len` returns the size of the `String` in bytes, which can be different from its length in characters.
> - `String::chars` returns an iterator over the actual characters.
> - When people refer to strings they could either be talking about `&str` or `String`.
> - When a type implements `Deref<Target = T>`, the compiler will let you transparently call methods from T.
> - `String` is implemented as a wrapper around a vector of bytes, many of the operations you see supported on vectors are also supported on `String`, but with come extra guarantees.

### Vec

`Vec` si the standard resizable heap-allocated buffer

[snippet](./snippets/src/rust_vec.rs)

`Vec` implements `Deref<Target = T>`, which means that you can call slice methods on a `Vec`.

> - `Vec` is a type of collection, along with `String` and `HashMap`. The data it contains is stored on the heap.
> - `vec![...]` is a canonical macro to use instead of `Vec::new()` and it supports adding initial elements to the vector.
> - To index the vector you use `[ ]`, but they **will panic if out of bounds**. Alternatively, using `get` will return an `Option`.

### HashMap

Standard hash map with protection agains HashDoS attacks

[snippet](./snippets/src/rust_hashmap.rs)

> - Unlike `vec!` there is no standard `hashmap!` macro.
> - Since Rust 1.56, HashMap implements `From<[(K, V); N]>`, which allows us to easily initialize a hash map from a literal array.
> - Alternatively, HashMap can be built from any `Iterator` which yields key-value tuples.

### Box

`Box` is an owned pointer to data on the heap.

[snippet](./snippets/src/rust_box.rs)

`Box<T>` implements `Deref<Target = T>`, which means that you can **call methods from T directly on a `Box<T>`**.

> - `Box` is like `std::unique_ptr` in C++, except that it's guaranteed to be not null.
> - A `Box` can be useful when you:
>   - have a type whose size that can't be known at compile time, but the Rust compiler wants to know an exact size.
>   - want to transfer ownership of a large amount of data. To avoid copying large amounts of data on the stack, instead store the data on the heap in a `Box` so only the pointer is moved.

#### Box with recursive data structures

Recursive data types with dynamic sizes need to use a `Box`

> implements a singlylinked list

[snippet](./snippets/src/rust_box_recursive_ds.rs)

> - The compiler need to compute a fixed size of the struct in memory. Recursive data structure would have an infinite size.
> - `Box` solves this problem as it has the same size as a regular pointer.

#### Niche optimization

A `Box` cannot be empty, so the pointer is always valid and non-`null`. This allows the compiler to optimize the memory layout.

### Rc

`Rc` is a reference-counted shared pointer. Use this when you need to refer to the same data from multiple places

[snippet](./snippets/src/rust_rc.rs)

- If you need to mutate the data inside an `Rc`, you will need to wrap the data in a type such as `Cell` or `RefCell`.
- See `Arc` if you are in a multi-threaded context.
- You can _downgrade_ a shared pointer into a `Weak` pointer to create a cycles that will get dropped.

> - `Rc`'s count ensures that its contained value is valid for as long as there are references.
> - `Rc` in Rust is like `std::shared_ptr` in C++.
> - `Rc::clone` is cheap: it creates a pointer to the same allocation and increases the reference count. Does not make a deep clone and can generally be ignored when looking for performance issues in code.
> - `make_mut` actually clones the inner value if necessary ("clone-on-write") and returns a mutable reference.
> - Use `Rc::strong_count` to check the reference count.
> - `Rc::downgrade` gives you a _weakly reference-counted_ object to create cycles that will be dropped properly (likely in combination with `RefCell`).

## Modules

We have seen how `impl` blocks let us namespace functions to a type.
Similarly, `mod` lets us namespace types and functions.

[snippet](./snippets/src/rust_module.rs)

> - Modules define organization and scope

### Visibility

Modules are a privacy boundary:

- Modules items are private by default (hides implementation details).
- Parent and sibling items are always visible.
- In other words, if an item is visible in module `foo`, it's visible in all the descendants of `foo`.

[snippet](./snippets/src/rust_module_visibility.rs)

> - Use `pub` keyword to make modules public.
> - Additionally, there are advanced `pub(...)` specifiers to restrict the scope of public visibility.
>   - [Documentation](https://doc.rust-lang.org/reference/visibility-and-privacy.html#pubin-path-pubcrate-pubsuper-and-pubself)
>   - Configuring `pub(crate)` visibility is a common pattern.
>   - Less commonly, you can give visibility to a specific path.
>   - In any case, visibility must be granted to an ancestor module (and all of its descendants).

### Paths

Paths are resolved as follows:

- As relative path:
  - `foo` or `self::foo` refers to `foo` in the current module,
  - `super::foo` refers to `foo` in the parent module.
- As an absolute path:
  - `crate::foo` refers to `foo` in the root of the current crate,
  - `bar::foo` refers to `foo` in the `bar` crate.

A module can bring symbols from another module into scope with `use`. You will typically see something like this at the top of each module.

### Filesystem hierarchy

The module content can be omitted:

```rust
mod garden;
```

The `garden` module content is found at:

- `src/garden.rs` (modern Rust 2018 style)
- `src/garden/mod.rs` (older Rust 2015 style)

Similarly, a `garden::vegetables` module can be found at:

- `src/garden/vegetables.rs` (modern Rust 2018 style)
- `src/garden/vegetables/mod.rs` (older Rust 2015 style)

The `crate` root is in:

- `src/lib.rs` (for a library crate)
- `src/main.rs` (for a binary crate)

Modules defined in files can be documented, too, using "inner doc comments". These document the item that contains them - in this case, a module.

```rust
//! This module implements the garden, including a highly performant germination
//! implementation

// Re-export types from this module.
pub use seeds::SeedPacket;
pub use garden::Garden;

/// Sow the given seed packets.
pub fn sow(seeds: Vec<SeedPacket>) { todo!() }

/// Harvest the produce in the garden that is ready.
pub fn harvest(garden: &mut Garden) { todo!() }
```

> - Rust will look for modules in `modulename/mod.rs` and `modulename.rs`, but this can be changed with a compiler directive:
>
> ```rust
> #[path = "some/path.rs"]
> mod some_module { }
> ```
>
> This is useful, for example, if you would like to place tests for a module in a file named `some_module_test.rs` similar to the convention in Go.

## Day-2 afternoon exercises
