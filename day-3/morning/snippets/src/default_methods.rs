trait Equals {
    fn equal(&self, other: &Self) -> bool;
    fn not_equal(&self, other: &Self) -> bool {
        !self.equal(other)
    }
}

#[derive(Debug)]
struct Centimeter(i16);

impl Equals for Centimeter {
    fn equal(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

// other structure might be
trait Equals2 {
    fn equal(&self, other: &Self) -> bool;
}

trait NotEqual: Equals2 {
    fn not_equal(&self, other: &Self) -> bool {
        !self.equal(other)
    }
}

#[derive(Debug)]
struct Millimeter(i16);

impl Equals2 for Millimeter {
    fn equal(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl NotEqual for Millimeter {}

// other structure might be
trait Equals3 {
    fn equal(&self, other: &Self) -> bool;
}

trait NotEqual2 {
    fn not_equal(&self, other: &Self) -> bool;
}

// blanket implementation of NotEqual2 for Equals3
impl<T> NotEqual2 for T
where
    T: Equals3,
{
    fn not_equal(&self, other: &Self) -> bool {
        !self.equal(other)
    }
}

#[derive(Debug)]
struct Micrometer(i16);
impl Equals3 for Micrometer {
    fn equal(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub fn main() {
    let a = Centimeter(10);
    let b = Centimeter(20);

    println!("{a:?} equals {b:?}: {}", a.equal(&b));
    println!("{a:?} not_equals {b:?}: {}", a.not_equal(&b));

    let a = Millimeter(10);
    let b = Millimeter(20);

    println!("{a:?} equals {b:?}: {}", a.equal(&b));
    println!("{a:?} not_equals {b:?}: {}", a.not_equal(&b));

    let a = Micrometer(10);
    let b = Micrometer(20);

    println!("{a:?} equals {b:?}: {}", a.equal(&b));
    println!("{a:?} not_equals {b:?}: {}", a.not_equal(&b));
}
