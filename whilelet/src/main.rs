fn main() {
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("I is greater than 9!");
            optional = None;
        } else {
            println!("i is {:?}, try again",i);
            optional = Some(i + 1);
        }
    }

    let mut j = 0;
    while j < 10 {
        println!("j is {}",j);
        j += 1;
    }
}
