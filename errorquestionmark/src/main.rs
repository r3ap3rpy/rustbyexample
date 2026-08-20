use std::num::ParseIntError;

fn multiply(x: &str, y: &str) -> Result<i32, ParseIntError> {
    let first = x.parse::<i32>()?;
    let second = y.parse::<i32>()?;
    Ok(first * second)
}
fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("The result: {}",n),
        Err(e) => eprintln!("The error: {}",e),
    }
}
fn main() {
    print(multiply("4", "3"));
    print(multiply("cica", "maca"));
}
