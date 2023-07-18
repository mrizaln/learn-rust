# Day 3 Morning

## Generics

### Generic data type

You can use generics to abstract over the concrete field type.

[snippet](./snippets/src/generics.rs)

### Generic methods

You can declare a generic type on your `impl` block.

[snippet](./snippets/src/generics.rs)

### Monomorphization

Generic code is turned into non-generic code based on the call sites:

```rust
fn main() {
    let integer = Some(5);
    let float = Some(5.0);
}
```

behaves as if you wrote

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

> Basically like template instantiation in C++

## Traits

Rust lets you abstract over types with traits. They're similar to interfaces (and possibly C++20 concepts).

[snippet](./snippets/src/traits.rs)

### Trait objects

Trait objects allow for values of different types, for instance in a collection.

[snippet](./snippets/src/trait_objects.rs)

> - Basically dynamic dispatch using vtable and stuff.

> - Types that implement a given trait may be of different sizes. This makes it impossible to have things like `Vec<Pet>` in the example above.
> - `dyn Pet` is a way to tell the compiler about a dynamically sized type that implements `Pet`.
> - In the example, `pets` holds _fat pointers_ to objects that implement `Pet`. The fat pointer consists of two components, a pointer to the actual object and a pointer to the virtual method table for the `Pet` implementation of that particular object.

### Deriving traits

You can let the compiler derive a number of traits.

```rust
#[derive(<list_of_traits>)]
```

[snippet](./snippets/src/deriving_traits.rs)

### Default methods

Traits can implement behavior in terms of other trait methods.

[snippet](./snippets/src/default_methods.rs)

> - Traits may specify pre-implemented (default) methods and methods that users are required to implement themselves. Methods with default implementations can rely on required methods.

### Trait bounds

When working with generics, you often want to require the types to implement some trait, so that you can call this trait's methods.

[snippet](./snippets/src/trait_bounds.rs)

> - Similar to concepts and require clause in C++ (?)
> - Open snippet for more information

### `impl` trait

Similar to trait bounds, an `impl Trait` syntax can be used in function arguments and return values

[snippet](./snippets/src/impl_trait.rs)

> Open snippet for more information

## Important Traits

- `Iterator` and `IntoIterator` used in `for` loops,
- `From` and `Into` used to convert values,
- `Read` and `Write` used for IO,
- `Add`, `Mul`, ... used for operator overloading, and
- `Drop` used for defining destructors.
- `Default` used to construct a default instance of a type.

### Iterators

You can implement the `Iterator` trait on your own types

[snippet](./snippets/src/iterators.rs)

> - The `Iterator` trait implements many common functional programming operations over collections (e.g. `map`, `filter`, `reduce`, etc). This is the trait where you can find all the documentation about them. In Rust, these functions should produce the code as efficient as equivalent imperative implementations.
> - `IntoIterator` is the trait that makes for loops work. It is implemented by collection types such as `Vec<T>` and references to them such as `&Vec<T>` and `&[T]`. Ranges also implement it. This is why you can iterate over a vector with `for i in some_vec { .. }` but `some_vec.next()` doesn't exist.

### FromIterator

`FromIterator` lets you buld a collection from an `Iterator`.

[snippet](./snippets/src/from_iterator.rs)

> - `Iterator` implements `fn collect<B>(self) -> B where B: FromIterator<Self::Item>, Self: Sized`.
> - There are also implementations which let you do cool things like convert an `Iterator<Item = Result<V, E>>` into a `Result<Vec<V>, E>`

### `From` and `Into`

Types implement `From` and `Into` to facilitate type conversions:

```rust
fn main() {
    let s = String::from("hello");
    let addr = std::net::Ipv4Addr::from([127, 0, 0, 1]);
    let one = i16::from(true);
    let bigger = i32::from(123i16);
    println!("{s}, {addr}, {one}, {bigger}");
}
```

`Into` is automatically implemented when `From` is implemented;

```rust
fn main() {
    let s: String = "hello".into();
    let addr: std::net::Ipv4Addr = [127, 0, 0, 1].into();
    let one: i16 = 123i16.into();
    println!("{s}, {addr}, {one}, {bigger}");
}
```

> - That's why it is common to only implement `From`, as your type will get `Into` implementation too.
> - When declaring a function argument input type like "anything that can be converted into a `String`", the rule is opposite, you should use `Into`. Your function will accept types that implement `From` and those that only implement `Into`.

### `Read` and `Write`

Using `Read` and `BufRead`, you can abstract over `u8` sources.
Similaryly, `Write` lets you abstract over `u8` sinks.

[snippet](./snippets/src/read_and_write.rs)

### The `Drop` trait

Values which implement `Drop` can specify code to run when they go out of scope (similar to Destructor in C++)

[snippet](./snippets/src/drop_trait.rs)

> - Why doesn't `Drop::drop` take `self`?
>   - Short answer: If it did, `std::mem::drop` would be called at the end of the block, resulting in another call to `Drop::drop`, and a stack overflow!

### The `Default` trait

`Default` trait produces a default value for a type.

[snippet](./snippets/src/default_trait.rs)

> - It can be implemented directly or it can be derived via `#[derive(Default)]`.
> - A derived implementation will produce a value where all fields are set to their default values.
>   - This means all types in the struct must implement `Default` too.
> - Standard Rust types often implement `Default` with reasonable values (e.g. `0`, `""`, etc).
> - The partial struct copy works nicely with default.
> - Rust standard library is aware that types can implement `Default` and provides convenience methods that use it.

### `Add`, `Mul`, ...

Operator overloading is implemented via traits in `std::ops`

[snippet](./snippets/src/operator_overload_trait.rs)

> - Why is `Output` an associated type? Could it be made a type parameter of the method?
>   - Short answer: Function type parameters are controlled by the caller, but associated types (like `Output`) are controlled by the implementator of a trait.
> - You could implement `Add` for two different types, e.g. `impl Add<(i32, i32)> for Point` would add a tuple to a `Point`.

### Closures

Closures or lambda expressions have types which cannot be named. However, they implemented special `Fn`, `FnMut`, and `FnOnce` traits.

[snippet](./snippets/src/closures.rs)

> - An `Fn` neither consumes nor mutates captured values, or perhaps captures nothing at all. It can be called multiple times concurrently.
> - An `FnMut` might mutate captured values. You can call it multiple times, but not concurrently.
> - If you have an `FnOnce`, you may only call it once. It might consume captured values.
> - `FnMut` is a subtype of `FnOnce`. `Fn` is subtype of `FnMut` and `FnOnce`.
> - The compiler also infers `Copy` and `Clone` depending on what the closure captures.
> - By default, closures will capture by reference if they can. The `move` keyword makes them capture by value.
