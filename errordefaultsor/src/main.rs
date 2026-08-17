#[derive(Debug)]
enum Fruits {
    Apple,
    Orange,
    Banana,
    Kiwi,
    Lemon,
}
fn main() {
    let apple = Some(Fruits::Apple);
    let banana = Some(Fruits::Banana);
    let no_fruit: Option<Fruits> = None;
    let first_available_fruit = no_fruit.or(banana).or(apple);
    println!("The first available fruit: {:?}",first_available_fruit);
}
