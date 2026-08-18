use std::io::{self, Write};

enum Possible {
    Even,
    Odd,
}

fn optional(x: i32) -> Option<Possible> {
    if x == 0 {
        return None;
    }
    if x % 2 == 0 {
        return Some(Possible::Even);
    } else {
        return Some(Possible::Odd);
    }
}

fn main() {
    let mut input = String::new();
    print!("Enter a number<i32>: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("could not read number!");
    let number = input.trim().parse().expect("Not a valid number!");
    match optional(number) {
        Some(Possible::Even) => println!("The number is even!"),
        Some(Possible::Odd) => println!("The number is odd!"),
        None => println!("I do not know!"),
    }
}
