# Day 2

## Structs

```rust
struct Person {
    name: String,
    age: u8
}

fn main() {
    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    println!("{} is {} years old", peter.name, peter.age);

    peter.age = 28;
    println!("{} is {} years old", peter.name, peter.age);

    let jackie = Person {
        name: String::from("Jackie"),
        ..peter                         // *
    };
    println!("{} is {} years old", peter.name, peter.age);
}
```

> - structs work like in C++ (no inheritance tho)
> - methods are defined in an impl block (covered in the [future]())
> - structs can have zero size: unit-like structs
>
> \*) this syntax allows us to copy the majority of the fields from the old struct without having to explicitly type it all out. it must be the last element.

<br />

### Tuple Structs

If the field names are unimportant, you can use a tuple struct

```rust
struct Point(i32, i32);

fn main() {
    let p = Point(17, 23)
}
```

Often used fro single-field wrappers (called newtypes)

```rust
struct PoundOfForce(f64);
struct Newtons(f64);

fn compute_thruster_force() -> PoundOfForce {
    todo!("Ask a rocket scientist at NASA")
}

fn set_thruster_force(force: Newtons) {
    // ...
}

fn main() {
    let force = compute_thruster_force();
    set_thruster_force(force)
}
```

> - Newtypes are a great way to encode additional information about the value in a primitive type

<br />

### Field shorthand syntax

If you already have variables with the right names, then you can create the struct using a shorthand:

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }                  // <<< using shorthand
        // Self { name, age }
    }
}

impl Default for Person {
    fn default() -> Person {
        Person {
            name: "Bot".to_string(),
            age: 0,
        }
    }
}

fn create_default() {
    let tmp = Person {
        ..Default::default()
    };
    println!("{tmp:?}");

    let tmp = Person {
        name: "Sam".to_string(),
        ..Default::default()
    };
    println!("{tmp:?}");
}

pub fn main() {
    let peter = Person::new(String::from("Peter"), 27);
    println!("{peter:?}");
}
```

<br />
<br />

## Enums

The `enum` keyword allows the creation of a type which has a few different variants

```rust
fn generate_random_number() -> i32 {
    4         // pretend this is random
}

#[derive(Debug)]
enum CoinFlip {
    Heads,
    Tails,
}

fn flip_coin() -> CoinFlip {
    let random_number = generate_random_number();
    if random_number % 2 == 0 {
        return CoinFlip::Heads;
    } else {
        return CoinFlip::Tails;
    }
}

fn main() {
    println!("You got: {:?}", flip_coin());
}
```

> we're able to define methods on enums (using impl same as struct)

<br />

### Variant payloads

You can define richer enums where the variants carry data (I want this in C++)

```rust
enum WebEvent {
    PageLoad,                 // variant without payload
    KeyPress(char),           // tuple struct variant
    Click { x: i64, y: i64 }, // full struct variant
}

#[rustfmt::skip]
fn inspect(event: WebEvent) {
    match event {     // no fallthrough
        WebEvent::PageLoad       => println!("page loaded"),
        WebEvent::KeyPress(c)    => println!("pressed '{c}'"),
        WebEvent::Click { x, y } => println!("clicked at x={x}, y={y}"),
    }
}

fn main() {
    use WebEvent;

    let load = PageLoad;
    let press = KeyPress('x');
    let click = Click { x: 20, y: 80 }:

    inspect(load);
    inspect(press);
    inspect(click);
}
```

> - match pattern similar to switch case (C++) but no fallthrough
> - match expression has value (like when Kotlin). the value is the last expression in the match arm (right side)
> - `match` inspects a hidden discriminant field in the `enum` (???)

<br />

### Enum sizes

Rust enums are packed tightly, taking constraints due to alignment into account[^type_layout]

[^type_layout]: https://doc.rust-lang.org/reference/type-layout.html

```rust
use std::mem::{align_of, size_of};

macro_rules! dbg_size {
    ($t:ty) => (
        println!("{}: size {} bytes, aligh: {} bytes",
          stringify!($t), size_of::<$t>(), align_of<$t>());
    )
}

enum Foo {
    A,
    B
}

fn main() {
    dbg_size!(Foo);     // Foo: size 1 bytes, align 1 bytes
}
```

> - Internally Rust is using a field (discriminant) to keep track of the enum variant
> - You can control the discriminant if needed
>
> ```rust
> #[repr(u32)]
> enum Bar {
>     A,    // 0
>     B = 10000,
>     C =   // 10001
> }
>
> fn main() {
>     println!("A: {}", Bar::A as u32);   // 0
>     println!("B: {}", Bar::B as u32);   // 10000
>     println!("C: {}", Bar::C as u32);   // 10001
> }
> ```
>
> - null pointer optimization: for [some types](https://doc.rust-lang.org/std/option/#representation), rust guarantees that `size_of::<T>()` equals `size_of::<Option<T>>()`

<br />

## Methods

Rust allows you to associate functions with your new types. You do this with an `impl` block

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn say_hello(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

fn main() {
    let peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    peter.say_hello();
}
```

<br />

### Method receiver

The `&self` above indicates that the method borrows the object immutably. There are other possible receivers for a method

- `&self`: borrows using shared and immutable reference
  > `const T const*` or `const T&` (possibly `const std::shared_ptr<T>` as well)
- `&mut self`: borrows using unique and mutable reference
  > no direct comparison -- `T*` or `T&` (possibly `std::shared_ptr<T>` as well) [not unique though]
- `self`: takes ownership, can't mutate
  > `std::unique_ptr<const T>` probably
- `mut self`: takes ownership and can mutate
  > `std::unique_ptr<T>`
- no receiver: become a static method

> there are also [special wrapper types](https://doc.rust-lang.org/reference/special-types-and-traits.html) allowed to be receiver types, such as `Box<Self>`

[Example](./snippets/src/example.rs)

<br />

## Pattern Matching

The `match` keyword let you match a value against one or more patterns. The comparisons are done from top to bottom and the first match wins.

```rust
fn main() {
    let input = 'x';

    match input {
        'q'                   => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),     // multiple value
        '0'..='9'             => println!("Number input"),      // range
        _                     => println!("Something else"),    // default branch
    }
}
```

> The \_ pattern is a wildcard which matches any value

<br />

### Destructuring enums

```rust
enum Result {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("cannot divide {n} into two equal parts"))
    }
}

fn main() {
    let n = 100;
    match divide_in_two(100) {
        Result::Ok(half) => println!("{n} divided in two is {half}"),
        Result::Err(msg) => println!("sorry, an error happened: {msg}"),
    }
}
```

<br />

### Destructuring structs

```rust
struct Foo {
    x: (u32, u32),
    y: u32,
}

#[rustfmt::skip]
fn main() {
    let foo = Foo { x: (1, 2), y: 3 };    // match first branch
    // let foo = Foo { x: (5, 2), y: 2 };    // match second branch
    // let foo = Foo { x: (3, 2), y: 5 };    // match third branch
    match foo {
        Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo { y: 2, x: i }   => println!("y = 2, x = {i:?}"),
        Foo { y, .. }        => println!("y = {y}, other fields were ignored"),
    }
}
```

<br />

### Destructuring arrays

You can destructure arrays, tuples, and slices by matching on their elements

```rust
#[rustfmt::skip]
fn main() {
    let triple = [0, -2, 3];
    println!("Tell me about {triple:?}");
    match triple {
        [0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
        [1, ..]   => println!("First is 1 and the rest were ignored"),
        _         => println!("All elements were ignored"),
    }
}
```

> destructuring of slices of unknown length also works with patterns of fixed length
>
> ```rust
> fn main() {
>    inspect(&[0, -2, 3]);                // match first branch
>    inspect(&[1, -2, 3, 4, 23, 24]);     // match second branch
>    inspect(&[0, -2, 3, 4, 23, 24]);     // match third branch
> }
>
> #[rustfmt::skip]
> fn inspect(slice: &[i32]) {
>    println!("Tell me about {slice:?}");
>    match slice {
>        &[0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
>        &[1, ..]   => println!("First is 1 and the rest were ignored"),
>        _          => println!("All elements were ignored"),
>    }
> }
> ```
>
> try match against the tail with patterns `[.., b]` and `[a@..,b]`

<br />

### Match guards

When matching, you can add a guard to a pattern. This is an arbitrary Boolean expression which will be executed if the pattern matches:

```rust
#[rustfmt:;skip]
fn main() P
    let pair = (2, -2);
    println!("Tell me about {pair:?}");
    match pair {
        (x, y) if x == y     => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _                    => println!("No correlation..."),
    }
```

> - Match guards as a separate syntax feature are important and necessary when we wish to concisely express more complex ideas than patterns alone would allow.
> - They are not the same as separate if expression inside of the match arm. An if expression inside of the branch block (after =>) happens after the match arm is selected. Failing the if condition inside of that block won’t result in other arms of the original match expression being considered.

<br />
<br />

## Exercises

<br />

### Health statistics

<br />

### Points and polygons
