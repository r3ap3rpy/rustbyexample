fn main() {
    println!("{} days ",31);
    println!("{0},{1},{2}","first","second","third");
    println!("Named formatting: {name}, {age} years old!",name="Daniel",age=35);

    println!("Decimal: {}",10);
    println!("Binary: {:b}",10);
    println!("Octal: {:o},",10);
    println!("Hex: {:x}",10);
    
    println!("Right justify: #{number:>10}#",number=10);
    println!("Left justify with padding character: #{number:z<10}#",number=20);

    #[allow(dead_code)]
    struct Zero;

    let number: f64 = 100.0;
    let width: usize = 10;
    println!("Captured with argument: #{number:z>width$}#");
}
