fn main() {

    let long_lived = 10;
    {
        let short_lived = "abc";
        println!("I only live in the block {}",short_lived);
        let long_lived = 20;
        println!("Shadowing the outer scope: {}",long_lived);
    }
    println!("I am long lived original: {}",long_lived);
}
