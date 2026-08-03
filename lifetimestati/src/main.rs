use std::fmt::Debug;

static NUM: i32 = 18;

fn print_it(input: impl Debug + 'static) {
    println!("Value passed was: {:?}",input);
}

fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}
fn main() {
    {
        let static_string = "I am in read-only memory!";
        println!("{}",static_string);
    }
    {
        let lifetime_num = 9;
        let coerced_static = coerce_static(&lifetime_num);
        println!("Coerced static: {}", coerced_static);
    }
    println!("NUM: {} stays accessible",NUM);

    let i = 5;
    print_it(i);
    //print_it(&i);
}
