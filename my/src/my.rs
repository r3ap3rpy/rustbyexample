mod inaccessible;
pub mod nested;

pub fn function() {
    println!("my.rs::function()");
}
fn private_function() {
    println!("my.rs::private_function()");
}
pub fn indirect_access() {
    println!("my.rs::indirect_access()");
    private_function();
}
