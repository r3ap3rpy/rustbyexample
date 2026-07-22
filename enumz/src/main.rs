enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click {x:i64, y:i64},
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded!"),
        WebEvent::PageUnload => println!("Page un-loaded!"),
        WebEvent::KeyPress(c) => println!("Key pressed {}!",c),
        WebEvent::Paste(s) => println!("Content pasted: {}!",s),
        WebEvent::Click {x,y} => println!("Location clicked: x: {} y: {} ",x,y),
    }
}

#[derive(Debug)]
enum MyVeryLongNamedEnum {
    first,
    second,
    third,
}

impl MyVeryLongNamedEnum {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::first => x + y,
            Self::second => x - y,
            Self::third => x * y,
        }
    }
}

type Order = MyVeryLongNamedEnum;

enum Stage {
    Beginner,
    Professional,
}
enum Role {
    Student,
    Teacher,
}

enum Numbers {
    one,
    two,
    three,
}

enum Colors {
    red = 0xff000,
    green = 0x00ff0,
    blue = 0x0000ff,
}

fn main() {
    let press = WebEvent::KeyPress('c');
    let pasted = WebEvent::Paste("my password".to_string());
    let clicked = WebEvent::Click{x:10,y:20};
    let loaded = WebEvent::PageLoad;
    let unloaded = WebEvent::PageUnload;

    inspect(loaded);
    inspect(press);
    inspect(pasted);
    inspect(clicked);
    inspect(unloaded);
    println!("{:?}",Order::first);
    println!("x + y = {}",Order::first.run(10,20));

    use Stage::{Beginner, Professional};
    use Role::*;

    let stage = Beginner;
    let role = Student;

    match stage {
        Beginner => println!("Novice"),
        Professional => println!("Whats there to improve"),
    }
    match role {
        Student => println!("Learning!"),
        Teacher => println!("Teaching!"),
    }

    println!("One is {}", Numbers::one as i32);
    println!("Red is {}", Colors::red as u32);
}
