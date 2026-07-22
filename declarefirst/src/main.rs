fn main() {
    let another_binding;

    {
        let x = 2;
        another_binding = x * x;
    }
    println!("Another binding: {}",another_binding);

    // variables must be initialized before use.
}
