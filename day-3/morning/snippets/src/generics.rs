#[allow(dead_code)]
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

struct TuplePoint<T>(T, T);

// generic implementation
impl<T> TuplePoint<T> {
    fn x(&self) -> &T {
        &self.0
    }

    fn set_x(&mut self, x: T) {
        self.0 = x;
    }
}

pub fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let mixed = Point { x: 1.0, y: 4 };

    println!("{integer:?}, {float:?}, and {mixed:?}");

    let mut p = TuplePoint(5, 10);
    println!("p.x = {}", p.x());
    p.set_x(23);
    println!("p.x = {}", p.x());
}
