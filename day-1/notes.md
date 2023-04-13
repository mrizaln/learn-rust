## Day-1

## What

- Rust is a new programming language which had its 1.0 release in 2015
- Statically compiled language
  - Uses LLVM as its backend
- Supports many platforms and arch
  - x86, ARM, WASM
  - Linux, Mac, Windows
- Used for wide range of devices
  - Firmware and boot loaders
  - Smart displays
  - Mobile phones
  - Desktops
  - Servers

## Unique selling points

- Compile time memory safety
- Lack of undefined runtime behavior
- Modern language features

## Compile time guarantess

Static memory management at compile time

- No uninitialized variables
- No memory leaks (mostly[^mem_leak])
- No double-frees
- No use-after-free
- No NULL pointers
- No forgotten locked mutexes
- No data races between threads
- No iterator invalidation

[^mem_leak]: see Box::leak, std::mem::forget

## Runtime guarantess

- Array access is bounds checked[^bound_check]
- Integer overflow is defined (via a compile time flag; create panic(debug), or wrap-around (release))

[^bound_check]: bounds checking can't be disabled with a compiler flag or unsafe keyword. however unsafe allows you to call functions such as slice::get_unchecked which does not do bounds checking

## Modern features

- Language features
  - Enums and pattern matching
  - Generics
  - No overhead FFI (foreign function interface)
  - Zero-cost abstractions
- Tooling
  - Great compiler errors
  - Built-in dependency manager
  - Built-in support for testing
  - Excellent LSP support

## Basic syntax

much of rust syntax will be similar to C/C++/Java

- blocks and scopes are delimited by curly braces (`{ ... }`)
- comments: `//` (single line), `/* ... */` (multi line)
- keywords like `if` and `while` work the same
- variable assignment is done with `=`, comparison with `==`

## Scalar types

```text
┏━━━━━━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ ━━━━━━━━━━━━━━━━━━━━━━ ┃ TYPES                          ┃ LITERALS               ┃
┡━━━━━━━━━━━━━━━━━━━━━━━━╇━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━╇━━━━━━━━━━━━━━━━━━━━━━━━┩
│ signed integers        │ i8, i16, i32, i64, i128, isize │ -10, 0 , 1_000, 123i64 │
├────────────────────────┼────────────────────────────────┼────────────────────────┤
│ unsigned integers      │ u8, u16, u32, u64, u128, usize │ 0, 123 10u16           │
├────────────────────────┼────────────────────────────────┼────────────────────────┤
│ floating point numbers │ f32, f64                       │ 3.14, -10.0e20, 2f32   │
├────────────────────────┼────────────────────────────────┼────────────────────────┤
│ strings                │ &str                           │ "foo", r##"\\"#        │
├────────────────────────┼────────────────────────────────┼────────────────────────┤
│ unicode scalar values  │ char                           │ 'a', 'α', '∞'          │
├────────────────────────┼────────────────────────────────┼────────────────────────┤
│ byte strings           │ &[u8]                          │ b"abc", br##" " "#     │
├────────────────────────┼────────────────────────────────┼────────────────────────┤
│ booleans               │ bool                           │ true, false            │
└────────────────────────┴────────────────────────────────┴────────────────────────┘
```

The types have width as follows

- `iN`, `uN`, `fN` are `N` bits wide
- `isize` and `usize` are the width of a pointer
- `char` is 32 bit wide
- `bool` is 8 bit wide

## Compound types

```text
┏━━━━━━━━┳━━━━━━━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ ━━━━━━ ┃ TYPES                   ┃ LITERALS                    ┃
┡━━━━━━━━╇━━━━━━━━━━━━━━━━━━━━━━━━━╇━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┩
│ arrays │ [T; N]                  │ [20, 30, 40], [0; 3]        │
├────────┼─────────────────────────┼─────────────────────────────┤
│ typles │ (), (T,), (T1, T2), ... │ (), ('x',), ('x', 1.2), ... │ *)
└────────┴─────────────────────────┴─────────────────────────────┘
```

\*) empty tuple can be imagined as void

> code: [snippets/compound_types.rs]

## References

like C++, rust has references

```rust
fn main() {
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x: {x}");
}
```

> - We must dereference ref_x when assigning to it, similar to C/C++ pointers
> - Rust will auto dereference in some cases, in particular when invoking method
> - References that are declared as mut can be bound to different values over their lifetime

Mutable reference or not?

```rust
let mut ref_x: &i32;        // mutable reference               (C++ pointer to const)
let mut ref_x: &mut i32;    // mutable reference to mutable    (C++ pointer)
let ref_x: &i32;            // reference                       (C++ const pointer to const)
let ref_x: &mut i32;        // reference to mutable            (C++ const pointer)
```

## Dangling references

Rust will statically forbid dangling references

```rust
fn main() {
    let ref_x: &i32;
    {
        let x: i32 = 10;
        ref_x = &x;                 // dangling reference
    }
    println!("ref_x: {ref_x}");
}
```

> - A reference is said to "borrow" the value it refers to
> - Rust is tracking the lifetimes of all references to ensure they live long enough
> - We will talk more about borrowing when we get to ownership

## Slices

A slice gives you a view into a larger collection

```rust
fn main() {
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];       // slice of 3rd to 4th element
    println!("s: {s:?}");

    let s: &[i32] = &a[..5];       // slice of 1st to 5th element
    println!("s: {s:?}");

    let s: &[i32] = &a[3..];       // slice of 1st to last element
    println!("s: {s:?}");

    let s: &[i32] = &a[..];       // slice of full array
    println!("s: {s:?}");
}
```

> Slices borrow data from the sliced type (reference)

## String vs str

```rust
fn main() {
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[6..];
    println!("s3: {s3}");
}
```

> - `&str` an immutable reference to a string slice (non-owning) `std::string_view` (C++), `const char*` (C/C++)
> - `String` a mutable string buffer (owning) `std::string` (C++) UTF-8 only and never small-string optimization
> - The `format!()` macro is a convenient way to generate an owned string from dynamic values. it accepts the same format specifications as `println!()`

## Functions

```rust
  fn main() {
      fizzbuzz_to(20);            // defined below, no forward declaration needed
  }

  fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
      if rhs == 0 {
          return false
      }
      lhs % rhs == 0              // last expression in a block is the return value (notice that there is no semicolon on this line)
  }

  fn fizzbuzz(n: u32) -> () {     // no return value means returning the unit type '()'
      match (is_divisible_by(n, 3), is_divisible_by(n, 5)) {
          (true, true)    => println!("fizzbuzz"),
          (true, false)   => println!("fizz"),
          (false, true)   => println!("buzz"),
          (false, false)  => println!("{n}"),
      }
  }

  fn fizzbuzz_to(n: 32) {         // `-> ()` is normally omitted
      for i in 1..=n {
          fizzbuzz(i);
      }
  }
```

## Rustdoc

All language items in rust can be documented using special `///` syntax

```rust
/// Determine whether the first argument is divisible by the second argument
///
/// if the second argument is zero, the result is false
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false
    }
    lhs % rhs == 0
}
```

> The content of rustdoc are treated as Markdown

## Methods

Rust has methods, they are simply functions that are associated with a particular type.
The first argument of a method is an instance of the type it is associated with.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }
}

fn main() {
    let mut rect = Rectangle { width: 10, height: 5 };
    println!("old area: {}"), rect.area());
    rect.inc_width(5);
    println!("old area: {}"), rect.area());
}
```

## Function overloading

Overloading is not supported (uhh... pain)

- Each function has a single implementation
  - Always takes a fixed number of parameters
  - Always takes a single set of parameter types
- Default values are not supported (shocked!)
  - All call sites have the same number of arguments
  - Macros are sometimes used as an alternative
- Function parameters can be generic

  ```rust
  fn pick_one<T>(a: T, b: T) -> T {
      if std::process::id() % 2 == 0 { a } else { b }
  }

  fn main() {
      println!("coin toss: {}", pick_one("heads", "tails"));
      println!("cash prize: {}", pick_one(500, 1000));
  }
  ```

## Exercices

- Implicit conversions

  Rust will not automatically apply implicit conversions between types

  - The rust integer types all implement the From<T> and Into<T> traits to let us convert between them
  - The From<T> trait has a sinlgle from() method and Into<T> has a single into() method

    > - [From](https://doc.rust-lang.org/std/convert/trait.From.html)
    > - [Into](https://doc.rust-lang.org/std/convert/trait.Into.html)

## Links

- [https://lib.rs/] -> find 3rd party libs
