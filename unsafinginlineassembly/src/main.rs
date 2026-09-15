use std::arch::asm;

fn main() {
    // basic usage
    unsafe {
        asm!("nop");
    }

    // input and output
    let x: u64;
    unsafe {
        asm!("mov {}, 5",out(reg) x);
    }
    println!("X = {}",x);

    let i: u64 = 3;
    let o: u64;
    unsafe {
        asm!(
            "add {0}, {1}, #5",
            out(reg) o,
            in(reg) i,
        );
    }
    assert_eq!(o, 8);
    println!("o = {}",o);


}
