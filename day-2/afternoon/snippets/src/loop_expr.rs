pub fn main() {
    let mut x = 10;

    let result = loop {
        x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };

        // println!("x: {x}");

        if x == 1 {
            break 42;
        }
    };
    println!("result: {result}")
}
