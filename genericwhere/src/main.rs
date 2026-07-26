use std::fmt::Debug;

trait PrintInOption {
    fn printinoption(self);
}

impl<T> PrintInOption for T where
    Option<T>: Debug {
        fn printinoption(self) {
            println!("{:?}",Some(self));
        }
    }

fn main() {
    let vector = vec![1,2,3,4,5];
    vector.printinoption();
}
