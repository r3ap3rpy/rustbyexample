#![allow(dead_code)]
use std::mem;
use rand::RngExt;
fn analyze_slice_size_and_length(slice: &[i32]) {
    println!("First element: {}",slice[0]);
    println!("Length of the slice: {}",slice.len());
}


struct Point2D {
    x: i32,
    y: i32,
}

impl Point2D {
    fn new(&self,x:i32,y:i32) -> Self {
        Self {
            x:x,
            y:y,
        }
    }
}
fn main() {
    let xs: [i32;5] = [1,2,3,4,5];
    let second_slice: [i32;400] = [0;400];

    
    let empty_array: [u32;0] = [];
    assert_eq!(&empty_array,&[]);
    assert_eq!(&empty_array,&[][..]);
    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("Value of index {} is {}", i, xval),
            None => println!("Slow down {} is too far!",i),
        }
    }
    analyze_slice_size_and_length(&second_slice);
    analyze_slice_size_and_length(&second_slice);
    println!("First element of the array: {}",xs[0]);
    println!("Second elemnt of the array: {}",xs[1]);
    println!("Number of elements: {}",xs.len());

    println!("Size in memory is {} bytes!",mem::size_of_val(&xs));
    let names = [
        "Daniel",
        "Gabriel",
        "Lenke",
        "Zenke",
    ];
    let mut vects: Vec<Point2D> = Vec::new();

    let mut rng = rand::rng();
    for i in 0..1_000 {
        let x: i32 = rng.random();
        let y: i32 = rng.random();
        let tmp = Point2D { x: x, y: y};
        println!("{} {}",tmp.x,tmp.y);
        vects.push(tmp);
    }
    println!("Size in memory is {} bytes!",mem::size_of_val(&vects));
    println!("Total memory {} bytes",mem::size_of::<Vec<Point2D>>() + vects.capacity() * mem::size_of::<Point2D>())    
}
