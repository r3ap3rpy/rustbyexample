#[allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct TuplePair(i32,i32);

#[derive(Debug)]
struct Point2D {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point2D,
    bottom_right: Point2D,
}
fn main() {
    let name = String::from("Daniel");
    let age = 35;
    let person = Person { name: name, age: age };
    println!("{:?}", person);

    let a = Point2D {x:5.2, y:0.4};
    let b = Point2D {x:10.3, y:0.2};
    println!("Point coordinates {{ x = {}, y = {} }}",a.x,a.y);

    let another_point = Point2D { x: 10.3, ..b};
    println!("{:?}",another_point);

    let Point2D {x:left_edge, y:top_edge} = a;

    let _unit = Unit;

    let pair = TuplePair(10,20);
    let rec = Rectangle {top_left: Point2D{x:10.0,y:20.0},bottom_right: Point2D{x:20.0,y:30.0}};
    println!("{:?}",rec);

    let recc = Rectangle { top_left: a, bottom_right: b};
    println!("{:?}",recc);
}
