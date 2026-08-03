fn elided_input(x: &i32) {
    println!("Elided input: {}",x);
}
fn annotated_input<'a>(x: &'a i32) {
    println!("Annotated input: {}",x);
}
fn elided_pass(x: &i32) -> &i32 { x }
fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
    x
}
fn main() {
    let x = 3;
    elided_input(&x);
    annotated_input(&x);
    println!("Elided pass: {}",elided_pass(&x));
    println!("Annotated pass: {}",annotated_pass(&x));
}
