#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let daniel = Person { name: "Daniel", age: 35};
    println!("{:?}",daniel);
    println!("{:#?}",daniel);
}
