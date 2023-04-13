enum Result {
    Ok(i32),
    Err(String),
}

struct Foo {
    x: (u32, u32),
    y: u32,
}

fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("cannot divide {n} into two equal parts"))
    }
}

#[rustfmt::skip]
pub fn main() {
    // enum
    let n = 100;
    match divide_in_two(100) {
        Result::Ok(half) => println!("{n} divided in two is {half}"),
        Result::Err(msg) => println!("sorry, an error happened: {msg}"),
    }
    println!();

    // struct
    let foo = Foo { x: (1, 2), y: 3 };    // match first branch
    // let foo = Foo { x: (5, 2), y: 2 };    // match second branch
    // let foo = Foo { x: (3, 2), y: 5 };    // match third branch
    match foo {
        Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo { y: 2, x: i }   => println!("y = 2, x = {i:?}"),
        Foo { y, .. }        => println!("y = {y}, other fields were ignored"),
    }
    println!();

    // array
    let triple = [0, -2, 3];
    println!("Tell me about {triple:?}");
    match triple {
        [0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
        [1, ..]   => println!("First is 1 and the rest were ignored"),
        _         => println!("All elements were ignored"),
    }
}
