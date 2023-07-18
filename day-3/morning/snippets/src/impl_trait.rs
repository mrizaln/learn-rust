use std::fmt::Display;

fn get_x(name: impl Display) -> impl Display {
    format!("Hello {name}")
}

// for parameter, impl Trait is like generic parameter with trait bound
// for return type, it means that the return type is some concrete type that implements the trait.

// // won't compile (unless add dyn for dynamic dispatch):
// fn get_x_not_working(name: Display) -> impl Display {
//     format!("Hello {name}")
// }

// compile
#[allow(dead_code)]
fn get_x_working<T: Display>(name: T) -> impl Display {
    format!("Hello {name}")
}

pub fn main() {
    let x = get_x("foo");
    println!("{x}");
}
