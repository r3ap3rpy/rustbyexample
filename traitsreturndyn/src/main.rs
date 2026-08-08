struct Cow;
struct Sheep;

trait Animal {
    fn noise(&self) -> &'static str;
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "Cow say's moooo!"
    }
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "Sheep says beeeh!"
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep{})
    } else {
        Box::new(Cow {})
    }
}
fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("{}",animal.noise());
}
