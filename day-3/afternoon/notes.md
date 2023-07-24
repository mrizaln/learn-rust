# Day 3 Afternoon

## Error Handling

Error handling in Rust is done using explicit control flow:

- Functions that can have errors list this in their return type,
- There are not exceptions.

### Panics

Rust will trigger a panic if a fatar error happens at runtime

```rust
fn main() {
    let v = vec![10, 20, 30];
    println!("v[100]: {}", v[100]);
}
```

> - Panics are for unrecoverable and unexpected errors.
>   - Panics are symptoms of bugs in the program.
> - Use non-panicking APIs (such as `Vec::get`) if crashing is not acceptable.

#### Catching the stack unwinding

By default, a panic will cause the stack to unwind. The unwinding can be caught.

[snippet](./snippets/src/stack_unwind.rs)

> - This does not work if `panic = 'abort'` is set in your `Cargo.toml`.

### Structured error handling with `Result`

The enum `Result` is used pervasively when errors are expected as part of normal operation.

[snippet](./snippets/src/error_handling_with_result.rs)

> - As with `Option`, the successful value sits inside of `Result`, forcing the developer to explicitly extract it. This encourages error checking. In the case where an error should never happen, `unwrap()` or `expect()` can be called, and this is a signal of the developer intent too.

### Propagating errors with `?`

The try-operator `?` is used to return errors to the caller. It lets you turn the common

```rust
match some_expression {
    Ok(value) => value,
    Err(err) => return Err(err),
}
```

into the much simpler

```rust
some_expression?
```

[snippet](./snippets/src/try_operator.rs)

#### Converting error types

The effective expansion of `?` is a little more complicated than previously indicated:

```rust
expression?
```

works the same as

```rust
match expression {
    Ok(value) => value,
    Err(err) => return Err(From::from(err)),
}
```

The `From::from` call here means we attempt to convert the error type to the type returned by the function

[snippet](./snippets/src/converting_error_types.rs)

#### Deriving from error enums

The [thiserror](https://docs.rs/thiserror/) crate is a popular way to create an error enum like we did on the previous page

[snippet](./snippets/src/thiserror_crate.rs)

#### Dynamic error types

Sometimes we want to allow any type of error to be returned without writing our own enum covering all the different possibilities. `std::error::Error` makes this easy

[snippet](./snippets/src/dynamic_error_type.rs)

> - It is generally not a good choice to use dynamic error for the public API of a library.

#### Adding context to errors

The widely used [anyhow](https://docs.rs/anyhow/) crate can help you add contextual information to your errors and allows you to have fewer custom error types

[snippet](./snippets/src/add_context_to_error.rs)

> - `anyhow::Result<V>` is a type alias for `Result<V, anyhow::Error`.
> - `anyhow::Error` is essentially a wrapper around `Box<dyn Error>`.
> - Actual error type inside of it can be extracted for examination if necessary.
> - Functionality provided by `anyhow::Result<T>` may be familiar to Go developers, as it provides similar usage patterns an ergonomics to `(T, error)` from Go.

## Testing

Rust and Cargo come with simple unit test framework:

- Unit tests are supported throughout your code.
- Integration tests are supported via the `tests/` directory.

### Unit tests

Mark unit tests with `#[test]`

```rust
fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(idx) => &text[..idx],
        None => &text,
    }
}

#[test]
fn test_empty() {
    assert_eq!(first_word(""), "");
}

#[test]
fn test_single_word() {
    assert_eq!(first_word("Hello"), "Hello");
}

#[test]
fn test_multiple_words() {
    assert_eq!(first_word("Hello world"), "Hello");
}
```

Use `cargo test` to find and run the unit tests.

### Test modules

Unit tests are often put in a nested module

```rust
fn helper(a: &str, b: &str) -> String {
    format!("{a} {b}");
}

pub fn main() {
    println!("{}", helper("Hello", "World"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helper() {
        assert_eq!(helper("foo", "bar"), "foo bar");
    }
}
```

> - This lets you unit test private helpers.
> - The `#[cfg(test)]` attribute is only active when you run `cargo test`.

### Documentation tests

Rust has built-in support for documentation tests

````rust
/// Shortens a string to the given length.
/// ```
/// use playground::shorten_string;
/// assert_eq!(shorten_string("Hello World", 5), "Hello");
/// assert_eq!(shorten_string("Hello World"), 20, "Hello World");
/// ```
pub fn shorten_string(s: &str, length: usize) -> &str {
    &s[..std::cmp::min(length, s.len())]
}
````

> - Code blocks in `///` comments are automatically seen as Rust code.
> - The code will be compiled and executed as part of `cargo test`.
> - Test the above code on Rust playground.

### Integration tests

If you want to test your library as a client, use an integration test.

Create a `.rs` file under `tests/`

```rust
user my_library::init;

#[test]
fn test_init() {
    assert!(init().is_ok());
}
```

These tests only have access to the public API of your crate.

### Useful crates

Rust comes with only basic support for writing tests.

Recommended crates:

- [googletest](https://docs.rs/googletest): comprehensive test assertion library in the tradition of GoogleTest for C++.
- [proptest](https://docs.rs/proptest): property-based testing for Rust.
- [rstest](https://docs.rs/rstest): support for fixtures and parameterised tests.

## Unsafe Rust

The Rust language has two parts:

- **Safe Rust**: memory safe, no undefined behavior possible.
- **Unsafe Rust**[^unsafe_rust]: can trigger undefined behavior if preconditions are violated.

Unsafe code is usually small and isolated, and its correctness should be carefully documented. It is usually wrapped in a safe abstraction layer.

Unsafe Rust gives you access to five new capabilities;

- Dereferencing raw pointers
- Access or modify mutable static variables.
- Access `union` fields.
- Call `unsafe` functions, including `extern` functions.
- Implement `unsafe` traits.

[^unsafe_rust]: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html

### Dereferencing raw pointers

Creating pointers is safe, but dereferencing them requires `unsafe`

[snippet](./snippets/src/dereferencing_raw_pointer.rs)

> - It is good practice to write a comment for each `unsafe` block explaining how the code inside it satisfies the safety requirements of the unsafe operations it is doing.
> - In the case of pointer dereferences, this means that the pointers must be valid[^pointer_valid], i.e.:
>   - The pointer must be non-null.
>   - The pointer must be _dereferencable_ (within the bounds of a single allocated object).
>   - The object must not have been deallocated.
>   - There must not be concurrent accesses to the same location.
>   - If the pointer was obtained by casting a reference, the underlying object must be live and no reference may be used to access the memory.
> - In most cases the pointer must also properly aligned.

[^pointer_valid]: https://doc.rust-lang.org/std/ptr/index.html#safety

### Mutable static variables

It is safe to read an immutable static variable

```rust
static HELLO_WORLD: &str = "Hello world!";

fn main() {
    println!("HELLO_WORLD: {HELLO_WORLD}");
}
```

However, since data races can occur, it is unsafe to read and write mutable static variables

```rust
static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    unsafe { COUNTER += inc; }    // potential data race!
}

fn main() {
    add_to_counter(42);

    unsafe { println!("COUNTER: {COUNTER}"); } // potential data race!
}
```

> - Using a mutable static is generally a bad idea, but there are some cases where it might make sense in low-level `no_std` code, such as Implementing a heap allocator or working with some C APIs.

### Unions

Unions are like enums, but you need to track the active field yourself

```rust
#[repr(C)]
union MyUnion {
    i: u8,
    b: bool,
}

fn main() {
    let u = MyUnion { i: 42 };
    println!("int: {}", unsafe { i.i });
    println!("bool: {}", unsafe { i.b });   // undefined behavior!
}
```

> - Unions are very rarely used -- they are occasionally needed for interacting with C library APIs.
> - If you just want to reinterpret bytes as different type, you probably want `std::mem::transmute` or a safe wrapper such as the `zerocopy` crate.

### Calling unsafe functions

A function or method can be marked unsafe if it has extra preconditions you must uphold to avoid undefined behavior

[snippet](./snippets/src/calling_unsafe_function.rs)

#### Writing unsafe functions

You can mark your own functions as `unsafe` if they require particular conditions to avoid undefined behavior.

[snippet](./snippets/src/writing_unsafe_function.rs)

> - Note that unsafe code is allowed within an unsafe function without an `unsafe` block. We can prohibit this with `#[deny(unsafe_op_in_unsafe_fn)]`

#### Calling extern functions

Function from other languages might violate the guarantees of Rust. Calling them is thus unsafe:

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        // undefined behavior if abs misbehave
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

> - This is usually only a problem for extern functions which do things with pointers which might violate Rust's memory model, but in general any C function might have undefined behavior under any arbritraty circumstances.
> - The `"C"` in this example is the ABI. Other ABIs are available too[^other_abi].

[^other_abi]: https://doc.rust-lang.org/reference/items/external-blocks.html

### Implementing unsafe traits

Like with functions, you can mark a trait as `unsafe` if the implementation must guarantee particular conditions to avoid undefine behavior.

For example, the `zerocopy` crate has an unsafe trait that looks something like this[^zerocopy_unsafe_trait]:

```rust
use std::mem::size_of_val;
use std::slice;

/// ...
/// # Safety
/// The type must have a defined representation and no padding.
pub unsafe trait AsBytes {
    fn as_bytes(&self) &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const Self as *const u8, size_of_val(self))
        }
    }
}

// Safe because u32 has a defined representation and no padding.
unsafe impl AsBytes for u32 {}
```

> - The should be a `# Safety` section on the Rustdoc for the trait explaining the requirements for the trait to be safely implemented.
> - The actual safety section for `AsBytes` is rather longer and more complicated.
> - The built-in `Send` and `Sync` traits are unsafe.

[^zerocopy_unsafe_trait]: https://docs.rs/zerocopy/latest/zerocopy/trait.AsBytes.html
