#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn say_hello(&self) {       // reference
        println!("Hello, my name is {}", self.name);
    }

    fn say_hello_consume(self) {            // move
        println!("Hello, my name is {}. Goodbye.", self.name);
    }
}

fn main() {
    let peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    peter.say_hello();
    peter.say_hello();
    peter.say_hello_consume();
    // peter.say_hello();                  // Illegal
}
