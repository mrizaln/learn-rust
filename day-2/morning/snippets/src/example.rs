#[derive(Debug)]
pub struct Race {
    name: String,
    laps: Vec<i32>,
}

#[rustfmt::skip]
impl Race {
    pub fn new(name: &str) -> Race {            // no receiver
        Race {
            name: String::from(name),
            laps: Vec::new(),
        }
    }

    pub fn add_lap(&mut self, lap: i32) {       // unique r/w access
        self.laps.push(lap);
    }

    pub fn print_laps(&self) {                  // ahared r/o acess
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    pub fn finish(self) {                       // exclusive ownership
        let total = self.laps.iter().sum::<i32>();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}

pub fn main() {
    let mut race = Race::new("Monaco Grand Prix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();
    // race.add_lap(42); // illegal
}
