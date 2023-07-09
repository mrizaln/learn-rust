pub fn main() {
    let five = Box::new(5);
    println!("five: {}", *five);
    println!("five: {}", five); // the operator* can be omitted thanks to the Deref trait
}
