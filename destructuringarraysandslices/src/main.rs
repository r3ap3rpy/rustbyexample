fn main() {
    let array = [1,-2,6];

    match array {
        [0,second,third] => println!("array[0] = 0, array[1] = {}, array[2] = {}",second,third),
        [9,_,third] => println!("array[1] = ignored, array[0] = 1, array[2] = {}",third),
        [-1,bind,..] => println!("array[1] = {} rest ignored", bind),
        [3, second, tail @ ..] => println!("array[1] = {}, Rest stored in slice: {:?}",second, tail),
        [first, middle @ .., last] => println!("First captured: {},Middle stored in slice {:?}, last captured: {}",first, middle, last),
    }
}
