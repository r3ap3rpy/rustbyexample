fn main() {
    println!("1 + 2 = {}",1u32 + 2);
    println!("10 - 4 = {}", 10u32 - 4);
    println!("1e4 is {}, -2.5e-3 is {}",1e4, -2.5e-3);

    println!("true and true is {}", true && true);
    println!("true or false is {}", true || false);
    println!("not false is {}", !false);

    println!("0011 AND 0101 is {:04b}",0b0011u32 & 0b0101u32);
    println!("0011 OR 0101 is {:04b}",0b0011u32 | 0b0101u32);
    println!("0011 XOR 0101 is {:04b}",0b0011u32 ^ 0b0101u32);
    println!("1 << 5 = {}", 1 << 5);
    println!("0x80 >> 2 = {:x}",0x80u32 >> 2);

    println!("One million is written as {}", 1_000_000u32);
}
