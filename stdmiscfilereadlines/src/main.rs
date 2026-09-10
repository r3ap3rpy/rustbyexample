use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("Cargo.toml") {
        for line in lines.map_while(Result::ok) {
            println!("Line: > {:?}",line);
        }
    }
}

fn read_lines<P>(file: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>{
    let file = File::open(&file)?;
    Ok(io::BufReader::new(file).lines())

}
