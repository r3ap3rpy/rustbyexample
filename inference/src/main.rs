fn main() {
    let x = 1u8;
    let mut vector = Vec::new();
    // The Vec<_> becomes Vec<u8> only after the first item is pushed
    // the compiler figures this out on the fly.
    vector.push(x);
    println!("{:?}",vector);
}
