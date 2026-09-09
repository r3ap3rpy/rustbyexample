use std::path::Path;

fn main() {
    let path = Path::new(".");
    let display = path.display();
    println!("{:?}",display);
    let mut new_path = path.join("a").join("b");
    new_path.push("c");
    new_path.push("d");
    println!("{:?}",new_path);
    if new_path.is_dir() {
        println!("Path does exist!");
    } else {
        println!("Path does NOT exist!");
    }
    new_path.set_file_name("whatever.tar.gz");
    println!("New path: {:?}",new_path);
    match new_path.to_str() {
        None => eprintln!("Does noot work!"),
        Some(s) => println!("Converted: {:?}",s),
    }
}
