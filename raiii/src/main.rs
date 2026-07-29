#[derive(Debug)]
struct Destructive;

impl Drop for Destructive {
    fn drop(&mut self) {
        println!("Calling destructor, freeing the resource!");
    }
}

fn create_box() {
    let _box1 = Box::new(5i32);

}

fn main() {
    let _box2 = Box::new(10i32);
    create_box();
    let x = Destructive;
    println!("{:?}",x);
    println!("Dropping!");
}
