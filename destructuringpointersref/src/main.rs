fn main() {
    let reference = &4;

    match reference {
        &val => println!("Got the value: {:?}",val),
    }

    match *reference {
        val => println!("Got the value: {:?}",val),
    }

    let _not_a_reference = 3;
    let ref _is_a_reference = 4;
    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Mutable value: {:?}",r),
    }

    match mut_value {
        ref mut m => {
            *m *= 10;
            println!("Multiplied reference mutable value: {:?}",m);
        }
    }
}
