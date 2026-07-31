fn main() {
    let i = 3;
    {
        let borrow = &i;
        println!("Borrowed: {}",borrow);
    }
    {
        let borrow_another = &i;
        println!("Another borrow: {}",borrow_another);
    }
    println!("i: {}",i);
}
