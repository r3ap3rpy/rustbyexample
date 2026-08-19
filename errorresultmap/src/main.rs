use std::num::ParseIntError;

fn multiply(x: &str, y: &str) -> Result<i32, ParseIntError> {
    match x.parse::<i32>() {
        Ok(first_number) => {
            match y.parse::<i32>() {
                Ok(second_number) => {
                    Ok(first_number * second_number)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e)
    }
}
fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(a) => println!("The result was: {}",a),
        Err(e) => eprintln!("The error was: {}",e),
    }
}
fn main() {
    let result = multiply("10", "11");
    print(result);
}
