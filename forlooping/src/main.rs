fn main() {
    for i in 1..=5 {
        println!("Ima inclusive: {}",i);
    }
    for i in 1..5 {
        println!("Ima not so inclusive: {}",i);
    }
    for i in (1..5).rev() {
        println!("Ima reversed: {}",i);
    }
    let borrowed = vec!["Bob","Daniel","Ferris"];
    for name in borrowed.iter() {
        match name {
            &"Ferris" => println!("Rustacean warning!"),
            _ => println!("hello {}",name),
        }
    }
    let consumed = vec!["Bob","Daniel","Ferris"];
    for name in consumed.into_iter(){
        match name {
            "Daniel" => println!("I am consumed, cannot be used anymore: {}",name),
            _ => println!("I cannot be used after this loop: {}", name),
        }
    }
    let mut inplace_modify = vec!["Bob".to_string(),"Daniel".to_string(),"Gabriel".to_string()];
    for name in inplace_modify.iter_mut() {
        let mut tmp = String::new();
        tmp.push_str(name);
        tmp.push_str("_in_place_modified!");
        *name = tmp;
    }
    println!("{:?}",inplace_modify);
}
