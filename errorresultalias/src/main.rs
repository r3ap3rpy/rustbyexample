use std::num::ParseIntError;

type AliasedResult<T> = Result<T, ParseIntError>;

fn multiply(x: &str, y: &str) -> AliasedResult<i32> {
    x.parse::<i32>().and_then(|x|{
        y.parse::<i32>().map(|y| x*y)
    })
}
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(a) => println!("The result: {}",a),
        Err(e) => eprintln!("The error: {}",e),
    }
}
fn main() {
    print(multiply("4","7"));
}
