use std::convert::From;
use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}


//impl From<i32> for Number {
//    fn from(item: i32) -> Self {
//        Number { value: item}
//    }
//}

impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self }
    }
}

fn main() {
    let my_str_slice = "my_string_slice";
    let my_string = String::from(my_str_slice);

    println!("My string: {}", my_string);
    //let my_number = Number::from(30);
    //println!("My number is {:?}",my_number);
    let int = 5;
    let num: Number = int.into();
    println!("My number into is {:?}",num);

    // From and Into are interchangeable 
}
