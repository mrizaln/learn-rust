pub fn main() {
    let mut v = vec![10, 20, 30];

    for x in v.iter_mut() {
        // *x = 1;
        println!("x: {x}");
    }

    for i in (0..10).step_by(2) {
        println!("i: {i}");
    }
}
