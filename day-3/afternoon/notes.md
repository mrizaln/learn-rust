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

#### Adding context to errors

## Testing

### Unit tests

### Test modules

### Documentation tests

### Integration tests

### Useful crates

## Unsafe Rust

### Dereferencing raw pointers

### Mutable static variables

### Unions

### Calling unsafe functions

### Implementing unsafe traits
