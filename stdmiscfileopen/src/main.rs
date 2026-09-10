use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("Cargo.toml");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(e) => panic!("Could not read the file: {:?}",e),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Could not read because: {:?}",why),
        Ok(_) => println!("{} contains: {:?}\n",display, s),
    }
}
