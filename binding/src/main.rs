
fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {
    println!("What kind of person are you?");
    match age() {
        0 => println!("You are not even born!"),
        n @ 1 ..=12 => println!("I am a child of age: {:?}",n),
        n @ 13 ..= 19 => println!("I am a teen of age(13-19): {:?}",n),
        n @ (1|7|15|13) => println!("I am a teen of age(1,7,15,13): {:?}",n),
        n => println!("I have nothign more to say: {:?}",n),
    }

    match some_number() {
        Some(n @ 42) => println!("The answer is {}!",n),
        Some(n) => println!("Not interessting: {}!",n),
        _ => (),

    }
}
