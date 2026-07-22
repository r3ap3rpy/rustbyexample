use std::fmt;

struct MyList(Vec<i32>);

impl fmt::Display for MyList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f,"[")?;

        for (i,v) in vec.iter().enumerate() {
            if i!=0 {
                write!(f,", ")?;
            }
            write!(f,"{}",v)?;
        }
        write!(f,"]")

    }
}

fn main() {
    let myvec = MyList(vec![1,2,3,4,5,6]);
    println!("{}",myvec);
}
