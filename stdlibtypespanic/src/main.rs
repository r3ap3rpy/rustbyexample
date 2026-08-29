fn division(x: i32, y: i32) -> i32 {
    if y == 0 {
        panic!("Cannot divide by zero!");
    } else {
        x / y
    }
}
fn main() {
   let _x = Box::new(0i32);
   division(3,0);
   println!("This point wont be reached!");
   // rustc panic.rs && valgrind ./panic
   // works only on linux or intel based macs
}
