fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap();
    2 * first.parse::<i32>().unwrap()
}
fn main() {
    let numbers = vec!["42","93","66"];
    //let empty = vec![];
    let strings = vec!["tofu","44","18"];
    println!("The first doubled: {}",double_first(numbers));
    //println!("The empty first doubled: {}",double_first(empty));
    //println!("The first doubled is : {}",double_first(strings));
}
