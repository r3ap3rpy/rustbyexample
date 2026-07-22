fn main() {
    let number = 5;
    match number {
        1 => println!("First!"),
        2|3|4|6|7|8 => println!("Almost there!"),
        19 => println!("Too much!"),
        _ => println!("Ima default handler!"),
    }
    let boolean = true;
    let binary = match boolean {
        true => 1,
        false => 0,
    };
    println!("The bool {} in binary is: {}",boolean, binary); 
}
