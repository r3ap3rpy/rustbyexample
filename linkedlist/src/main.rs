use crate::List::*;
enum List {
    Cons(u32,Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }
    fn len(&self) -> u32 {
        match *self {
            Cons(_,ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => format!("{} {}",head, tail.stringify()),
            Nil => format!("Nil"),
        }
    }
}
fn main() {
    let mut my_list = List::new();
    my_list = my_list.prepend(1);
    my_list = my_list.prepend(2);
    my_list = my_list.prepend(3);
    my_list = my_list.prepend(4);
    println!("The length of the list: {}",my_list.len());
    println!("{}",my_list.stringify());
}
