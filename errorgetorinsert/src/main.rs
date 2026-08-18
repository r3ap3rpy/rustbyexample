#[derive(Debug)]
enum Fruits {
    Apple,
    Banana,
    Kiwi,
    Lemon
}

fn main() {
    let mut my_fruit: Option<Fruits> = None;
    let apple = Fruits::Apple;
    let first_available_fruit = my_fruit.get_or_insert(apple);
    println!("first: {:?}",first_available_fruit);
    println!("my_fruit: {:?}",my_fruit);
}
