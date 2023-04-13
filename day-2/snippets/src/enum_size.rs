use std::mem::{align_of, size_of};

macro_rules! dbg_size {
    ($t:ty) => {
        println!("{}: size {} bytes, align: {} bytes",
                 stringify!($t), size_of::<$t>(), align_of::<$t>());
    };
}

enum Foo {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
}

#[repr(u32)]
enum Bar {
    A,    // 0
    B = 10000,
    C,    // 10001
}


fn main() {
    dbg_size!(Foo);
    dbg_size!(Bar);
    println!("A: {}", Bar::A as u32);
    println!("B: {}", Bar::B as u32);
    println!("C: {}", Bar::C as u32);
}
