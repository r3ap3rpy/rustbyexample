macro_rules! say_hello {
    () => {
        println!("Hello Macro!")
    };
}
fn main() {
    say_hello!()
}
