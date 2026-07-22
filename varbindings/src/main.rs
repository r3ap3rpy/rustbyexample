fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();
    let copied_integer = an_integer;

    let _unused_silenced = 10;

    
    println!("Integer {}",an_integer);
    println!("Boolean {}",a_boolean);
    println!("Unit {:?}",unit);
    println!("Copied integer {}",copied_integer); 
}
