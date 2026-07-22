fn main() {
    let mytriplet = (1,2,3);
    println!("{:?}",mytriplet);
    match mytriplet {
        (1,y,z) => println!("First is 1, then {} then {}",y,z),
        (1,..) => println!("Only first matters!"),
        (..,3) => println!("Only last matters!"),
        (1,..,3) => println!("The middle does not matter!"),
        _ => println!("Cannot destructure!"),
    }
}
