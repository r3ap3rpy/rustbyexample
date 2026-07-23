pub fn public_function() {
	println!("lib.rs public_function()");
}
fn private_function() {
	println!("lib.rs private_function()");
}
pub fn indirect_access() {
	println!("lib.rs indirect_access()");
	private_function();
}
// rustc --crate-type=lib rary.rs
// this prefixes the this with "lib" so the output is "library.rs"
