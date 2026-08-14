fn drink_some(beverage: &str) {
    if beverage == "beer" || beverage == "wine" {
        println!("Drinking some: {}",beverage);
    }
    panic!("Nasty drink")
}
fn main() {
   drink_some("wine");
   drink_some("beer");
   drink_some("lemonade");
}
