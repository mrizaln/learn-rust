//! you can do this with T: Trait or impl Trait

fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

// alternative syntax (declutters function signature)
// has additional features making it more powerful:
//      the type on the left of ":" can be arbitraty, like Option<T>
#[allow(dead_code)]
fn duplicate_alt<T>(a: T) -> (T, T)
where
    T: Clone,
{
    (a.clone(), a.clone())
}

// syntactic sugar for:
//      fn add_42_millions<T: Into<i32>>(x: T) -> i32 {
fn add_42_millions(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}

// struct NotClonable;

pub fn main() {
    let foo = String::from("foo");
    let pair = duplicate(foo);
    println!("{pair:?}");

    let many = add_42_millions(42_i8);
    println!("{many}");
    let many_more = add_42_millions(10_000_000);
    println!("{many_more}");
}
