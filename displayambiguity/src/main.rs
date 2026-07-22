use std::fmt;

#[derive(Debug)]
struct MinMax(i32,i32);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"({}, {})",self.0,self.1)
    }
}

#[derive(Debug)]
struct Point2D{
    x: i32,
    y: i32,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) ->fmt::Result {
        write!(f,"x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let mim = MinMax(10,20);
    println!("Compare structure!");
    println!("Dispaly: {}", mim);
    println!("Debug: {:?}",mim);

    let big_range = MinMax(-300,300);
    let small_range = MinMax(-3,3);

    println!("The big range is {big_range}, small range is {small_range}",big_range = big_range, small_range = small_range);

    let point = Point2D {x:1,y:23};
    println!("Compare structure!");
    println!("Display: {}",point);
    println!("Debug: {:?}",point);
}
