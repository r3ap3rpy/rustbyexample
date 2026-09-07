use std::thread;

const NTHREADS: u32 = 8;

fn main() {
    let mut children = vec![];
    for i in 0..NTHREADS {
        children.push(thread::spawn(move||{
            println!("This is thread number: {}",i);
        }));
    }
    for child in children {
        let _ = child.join();
    }
}
