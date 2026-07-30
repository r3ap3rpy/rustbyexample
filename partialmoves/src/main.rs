#[derive(Debug)]
struct Person {
    name: String,
    age: Box<u8>,
}

fn main() {
    let daniel = Person {
        name: String::from("Daniel"),
        age: Box::new(35u8),
    };

    let Person {name, ref age} = daniel;
    println!("Name: {}, age: {}",name,age);
    
    // the age can be used but name was moved out of the person struct
    println!("Daniel's age is {}",daniel.age);

}
