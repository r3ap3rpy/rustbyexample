#[derive(Debug)]
struct Point {x: i32, y: i32, z: i32}

fn main() {
    let mut point = Point {x: 1, y: 2, z: 3};

    let borrowed = &point;
    let another_borrowed = &point;

    println!("x: {}, y: {}, z: {}",point.x, borrowed.y, another_borrowed.z);

    let mutable_borrow = &mut point;
    mutable_borrow.x = 10;
    mutable_borrow.y = 20;
    mutable_borrow.z = 30;

    println!("{:?}",mutable_borrow);
}
