// growable string like and vectors
// optional types Option<T>
// error handling types Result<T,E>
// heap allocated Box<T> smart pointer pointing to the values in stack
use std::mem;

#[allow(dead_code)]
#[derive(Debug,Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}
fn origin() -> Point {
    Point {x:0.0,y:0.0}
}
fn boxed_origin() -> Box<Point> {
    Box::new(Point{x:0.0,y:0.0})
}
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}
fn main() {
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: -3.0, y: -1.0 }
    };
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: -4.0, y: -2.0}
    });
    let boxed_point: Box<Point> = Box::new(origin());
    let box_in_a_box: Box<Box<Point>> = Box::new(Box::new(origin()));
    println!("Point occupies {} bytes on the stack",mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes on the stack",mem::size_of_val(&rectangle));
    println!("Boxed point occupies {} bytes on the stack",mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes on the stack",mem::size_of_val(&boxed_rectangle));
    println!("Boxed box occupies {} bytes on the stack",mem::size_of_val(&box_in_a_box));
    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes on the stack",mem::size_of_val(&unboxed_point));
}
