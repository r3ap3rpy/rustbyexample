fn main() {
    let mut count: i32 = 0;
    loop {
        println!("Count is :: {}",count);
        if count > 100 {
            break;
        }
        if count % 2 != 0 {
            println!("Skipping odd!");
            count += 1;
            continue;
        }
        count += 1;

    }
    println!("Hello, world!");
}
