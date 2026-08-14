#[cfg(panic = "unwind")]
fn ah() {
    println!("Spit it out!");
}
#[cfg(not(panic = "unwind"))]
fn ah() {
    println!("This is not your party, run!");
}

fn drink_beverage(beverage: &str) {
    if beverage == "lemonade" {
        ah();
    } else {
        println!("Some refreshing {} is all I need!",beverage);
    }
}
fn main() {
    drink_beverage("lemonade");
    drink_beverage("beer");
    // With abort, a panic immediately terminates the process. There is no stack unwinding and destructors aren't run as part of unwinding.
    // When a panic happens, Rust walks back up the stack, running destructors (Drop) for values as it goes.
    // default is unwind
    // rustc main.rs -C panic=unwind
    // rustc main.rs -C panic=abort
}
