use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
#[allow(dead_code)]
struct Node {
    value: i64,
    parent: Option<Weak<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
}

#[allow(unused_mut)]
pub fn main() {
    // simple Rc use
    let mut a = Rc::new(10);
    let mut b = Rc::clone(&a);

    println!("a: {a}");
    println!("b: {b}");

    // more complicated example I guess
    let mut root = Rc::new(RefCell::new(Node {
        value: 42,
        parent: None,
        children: vec![],
    }));
    let child = Rc::new(RefCell::new(Node {
        value: 43,
        children: vec![],
        parent: Some(Rc::downgrade(&root)),
    }));
    root.borrow_mut().children.push(child);

    println!("graph: {root:#?}");
}
