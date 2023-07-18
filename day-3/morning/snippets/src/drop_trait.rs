struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

#[allow(unused_variables)]
pub fn main() {
    let a = Droppable { name: "a" };
    {
        let b = Droppable { name: "b" };
        {
            let c = Droppable { name: "c" };
            let d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Exiting block A");
    }
    // a.drop(); // explicit destructor calls not allowed
    drop(a); // "call" destructor (see docs of it. quite funny actually)

    println!("Exiting main");
}
