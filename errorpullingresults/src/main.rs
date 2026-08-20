use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Result<Option<i32>,ParseIntError> {
    let opt = vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    });

    opt.transpose()
}
fn main() {
    let numbers = vec!["33","44","55"];
    let empty = vec![];
    let strings = vec!["tofu","66","77"];
    println!("The first doubled is {:?}", double_first(numbers));
    println!("The first doubled is {:?}", double_first(empty));
    println!("The first doubled is {:?}", double_first(strings));
}
