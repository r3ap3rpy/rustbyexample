#[derive(Debug)]
enum Fruits {
    Apple,
    Banana,
    Kiwi,
    Lemon
}
fn main() {
    let mut my_fruit: Option<Fruits> = None;
    let get_lemon_as_fallback = || {
        println!("Lemon fallback!");
        Fruits::Lemon
    };
    let first_fruit = my_fruit.get_or_insert_with(get_lemon_as_fallback);
    println!("first: {:?}",first_fruit);
    println!("my fruit: {:?}",my_fruit);
}
