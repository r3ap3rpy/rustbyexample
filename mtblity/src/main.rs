fn main() {
    let _immutable_binding: i32 = 1;
    let mut mutable_binding: i32= 10;
    println!("The mutable {} and immutable {}",_immutable_binding, mutable_binding);
    mutable_binding += 10;
    println!("New value for the mutable {}",mutable_binding);
}
