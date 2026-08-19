use std::num::ParseIntError;

fn multiply(x: &str, y: &str) -> Result<f64,String> {
    let first = x.parse::<f64>().unwrap();
    let second = y.parse::<f64>().unwrap();
    Ok(first * second)
}
fn main() -> Result<(), ParseIntError> {
    let result = multiply("10", "20").unwrap();
    println!("The result was: {}", result);
    let number = "20";
    let number_parsed = match number.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };
    println!("The parsed number was: {}",number_parsed);
    Ok(())
}
