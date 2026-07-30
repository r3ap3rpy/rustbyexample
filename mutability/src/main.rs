fn main() {
    let immutable_box = Box::new(5i32);
    println!("Immutable box: {}",immutable_box);

    let mut mutable_box = immutable_box;

    *mutable_box = 10i32;
    println!("Mutable box: {}",mutable_box);
}
