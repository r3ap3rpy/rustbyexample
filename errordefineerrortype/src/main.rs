use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug,Clone)]
struct DoubleError;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid item to double!")
    }
}

fn double_vec(vec: Vec<&str>) -> Result<i32> {
    vec.first().ok_or(DoubleError).and_then(|s|{
        s.parse::<i32>().map_err(|_| DoubleError).map(|i| i * 2)
    })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The result was: {}",n),
        Err(e) => eprintln!("The error was: {}",e),
    }
}

fn main() {
    let numbers = vec!["11","22","33"];
    let empty = vec![];
    let strings = vec!["tofu","44","55"];
    print(double_vec(numbers));
    print(double_vec(empty));
    print(double_vec(strings));
}
