fn checked_division(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y) 
    }
}
fn try_division(dividend: f64, divisor: f64) {
    match checked_division(dividend, divisor) {
        None => eprintln!("{} / {} failed!",dividend, divisor),
        Some(quotient) => println!("{} / {} = {}",dividend,divisor,quotient),
    }
}
fn main() {
    match checked_division(11.0, 22.0) {
        Some(n) => println!("The result was: {}",n),
        None => println!("You cannot divide by zero!"),
    }
    try_division(10.0, 2.0);
}
