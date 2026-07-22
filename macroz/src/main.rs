use macroz::{time_this, time_this_too, to_do};
use std::{todo,unimplemented};

fn summa(x: i32, y: i32) -> i32 {
    x + y
}
fn main() {

    time_this! {
        println!("Hello World!");
    }

    time_this_too! {
        let mut nums: Vec<i32> = vec![1000;1000];
        nums.iter_mut().for_each(|n| { *n *= 3;});
        let sum: i32 = nums.iter().sum();
        dbg!(sum);
    }
    to_do!{
        let criticality = "CRITICAL";
        let message = "Revisit the algorithm!";
        println!("This is the sum: {}",summa(10,20));
    }
    //todo!("This is to be done!");
    unimplemented!("This is so cool!");
}
