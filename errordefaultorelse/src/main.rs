#[derive(Debug)]
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let no_fruit: Option<Fruit> = None;
    let kiwi_as_fallback = || {
        println!("Kiwi as fallback!");
        Some(Fruit::Kiwi)
    };
    let lemon_as_fallback = || {
        println!("Lemon as fallback!");
        Some(Fruit::Lemon)
    };
    let first_available = no_fruit.or_else(kiwi_as_fallback).or_else(lemon_as_fallback);
    println!("First available: {:?}",first_available);
}
