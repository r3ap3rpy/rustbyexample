fn eat_box_i32(input: Box<i32>) {
    println!("Consumed: {}",input);
}

fn borrow_i32(input: &i32) {
    println!("Just borrowing: {}",input);
}
fn main() {
    let a = Box::new(10i32);
    let b = 20i32;

    borrow_i32(&a);
    borrow_i32(&b);

    {
        let _ref_to_i32: &i32 = &a;
        //eat_box_i32(a);
        borrow_i32(&_ref_to_i32);
    }
    eat_box_i32(a);
}
