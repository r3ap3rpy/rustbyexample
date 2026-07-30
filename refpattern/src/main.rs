#[derive(Debug,Copy,Clone)]
struct Point {x: i32, y: i32}

fn main() {
    let c = 'Q';
    // the below 2 are equal
    let ref c_q = c;
    let cqq = &c;

    println!("{}",*c_q == *cqq);

    let point = Point {x: 10, y: 20};

    let _copy_of_x = {
        let Point {x: ref ref_to_x, y: _} = point;
        *ref_to_x
    };
    let mut mutable_point = point;
    {
        let Point {x:_, y: ref mut mut_ref_to_y } = mutable_point;
        *mut_ref_to_y = 1
    }

    println!("Point is ({},{}) ",point.x,point.y);

    let mut tuple = (Box::new(1i32),2u32);

    {
        let (_, ref mut last) = tuple;
        *last = 2u32;
    }
    println!("The tuple is  {:?}",tuple);

}
