use std::num::ParseIntError;

fn multiply(x: &str, y: &str) -> Result<i32, ParseIntError> {
    let first = match x.parse::<i32>() {
        Ok(first) => first,
        Err(e) => return Err(e),
    };
    let second = match y.parse::<i32>() {
        Ok(second) => second,
        Err(e) => return Err(e),
    };
    Ok(first * second)
}

fn print(result: Result<i32,ParseIntError>) {
    match result {
        Ok(a) => println!("The result: {}",a),
        Err(e) => eprintln!("The error: {}",e),
    }
}

fn main() {
    print(multiply("10","11"));
}
