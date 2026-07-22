fn main() {
    // statement
    // statement
    // statement

    // variable binding
    let x = 1;
    x;
    15;
    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_qubed = x_squared * x;
        x_qubed + x_squared + x
    };
    println!("The value of y: {}", y);
}
