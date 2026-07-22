fn main() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 1;
    let j = 1.0;

    println!("The size of 'x' in bytes is {}",std::mem::size_of_val(&x));
    println!("The size of 'y' in bytes is {}",std::mem::size_of_val(&y));
    println!("The size of 'z' in bytes is {}",std::mem::size_of_val(&z));
    println!("The size of 'i' in bytes is {}",std::mem::size_of_val(&i));
    println!("The size of 'j' in bytes is {}",std::mem::size_of_val(&j));
}
