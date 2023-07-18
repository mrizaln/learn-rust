trait Pet {
    fn name(&self) -> String;
}

struct Dog {
    name: String,
}

struct Cat;

impl Pet for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Pet for Cat {
    fn name(&self) -> String {
        String::from("The cat") // no name, cats won't respond to it anyway
    }
}

fn greet<P: Pet>(pet: &P) {
    println!("Who's a cutie? {} is!", pet.name());
}

pub fn main() {
    let fido = Dog {
        name: "Fido".into(),
    };
    greet(&fido);

    let captain_floof = Cat;
    greet(&captain_floof);
}
