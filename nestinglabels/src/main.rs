#[allow(unused_labels,unreachable_code)]
fn main() {
    'outer: loop {
        println!("Outer loop initiated!");
        'inner: loop {
            println!("Inner loop entered!");
            break 'outer;
        }
        println!("This was never reached!");
    }
    println!("Exited outer loop!");
}
