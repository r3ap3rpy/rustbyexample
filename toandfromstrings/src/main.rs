use std::fmt::{Display, Formatter, Result as RResult};
use std::str::FromStr;
use std::num::ParseIntError;

struct Circle {
    radius: i32,
}

impl Display for Circle {
    fn fmt(&self, f: &mut Formatter) -> RResult {
        write!(f,"The Circle radius is: {}",self.radius)
    }
}

impl FromStr for Circle {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> { 
        match s.trim().parse() {
            Ok(num) => Ok(Circle { radius: num}),
            Err(e) => Err(e),
        }
    }
}

fn main() {
    let my_circle = Circle { radius: 6 };
    println!("my_circle: {}",my_circle);

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("Parsed sum: {}",sum);

    let parsed_radius = "  3  ";
    let circle: Circle = parsed_radius.parse().unwrap();
    println!("My parsed circle: {}", circle);
}
