mod my;
fn function() {
    println!("my::function()");
}
fn main() {
    my::function();
    function();
    my::indirect_access();
    my::nested::function();
}
