#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }                        // using shorthand
        // Self { name, age }
    }
}

impl Default for Person {
    fn default() -> Person {
        Person {
            name: "Bot".to_string(),
            age: 0,
        }
    }
}

fn create_default() {
    let tmp = Person {
        ..Default::default()
    };
    println!("{tmp:?}");

    let tmp = Person {
        name: "Sam".to_string(),
        ..Default::default()
    };
    println!("{tmp:?}");
}

pub fn main() {
    create_default();

    let peter = Person::new(String::from("Peter"), 27);
    println!("{peter:?}");
}
