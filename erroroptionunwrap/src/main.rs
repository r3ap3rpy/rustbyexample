fn give_adult(drink: Option<&str>) {
    match drink {
        Some("lemonade") => println!("Yuck, too sugary!"),
        Some(inner) => println!("{} will suffice",inner),
        None => println!("At least give me something!"),
    }
}
fn drink(drink: Option<&str>) {
    let inside = drink.unwrap();
    if inside == "lemonade" { panic!("AaaaaAAaaaa"); }
    println!("I love {}",inside);
}
fn main() {
    let water = Some("water");
    let lemonade = Some("lemonade");
    let void = None;
    give_adult(water);
    give_adult(lemonade);
    give_adult(void);
    let coffee = Some("coffee");
    let nothing = None;
    drink(coffee);
    drink(nothing);
}
