use std::env;

fn increase(number: i32)  {
    println!("{}", number + 1);
}
fn decrease(number: i32) {
    println!("{}", number - 1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("My path is: {}",args[0]);
    println!("I got {:?} arguments: {:?}",args.len() - 1, &args[1..]);

    match args.len() {
        1 => println!("There were no arguments..."),
        2 => {
            match args[1].parse() {
                Ok(42) => println!("This is the answer!"),
                _ => println!("Not the answer!"),
            }
        },
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            let number: i32 = match num.parse() {
                Ok(n) => n,
                Err(_) => {eprintln!("Please provide a number!");return;}
            };
            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    eprintln!("Unknown command!");
                    return;
                }
            }
        },
        _ => println!("Cant handle these many arguments!"),
    }

}
