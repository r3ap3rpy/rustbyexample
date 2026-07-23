use std::mem;

fn apply<F>(f: F)
where 
    F: FnOnce() {
        f();
}

fn apply_to_3<F>(f: F) -> i32
where 
    F: Fn(i32) -> i32 {
        f(3)
}

fn fn_apply<F>(f: F)
where
    F: Fn() {
        f();
}

fn main() {
    let outer_variable = 42;
    let closure_annotated = | i: i32 | -> i32 { i + outer_variable };
    let closure_not_annotated = | i | { i + outer_variable };

    println!("Closure annotated: {}", closure_annotated(10));
    println!("Closure inferrred: {}", closure_not_annotated(5));

    let one = || 1;
    println!("The one(): {}",one());

    let color = String::from("green");

    // print only holds an immutable reference
    let print = || println!("color: {}",color);
    print();
    let _reborrow = &color;
    let color_moved = color;

    let mut count = 0;
    let mut inc = ||  {
        count += 1;
        println!("Count: {}",count);
    };
    inc();
    inc();
    inc();

    let movable = Box::new(3);
    let consume = || {
        println!("Movable: {:?}",movable);
        drop(movable);
    };

    consume();

    let haystack = vec![1,2,3];
    let contains = move |needle| haystack.contains(needle);
    println!("Haystack contains 2 : {}",contains(&2));

    let greeting = "Hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}",greeting);
        farewell.push_str("!!!");
        println!("Then I screamed {}",farewell);
        println!("Now I can sleep!");
        drop(farewell);
    };
    apply(diary);
    let double = |x| x*2;
    println!("3 doubled is {}",apply_to_3(double));

    let x = 8;
    let printer = || println!("x: {}",x);
    fn_apply(printer);
}
