use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
#[cfg(target_family="unix")]
use std::os::unix;
#[cfg(target_family="windows")]
use std::os::windows;
use std::path::Path;

fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Err(why) => Err(why),
        Ok(_) => Ok(s),
    }
}

fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;
    f.write_all(s.as_bytes())
}

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(why) => Err(why),
    }
}
fn main() {
    match fs::create_dir("a") {
        Ok(_) => println!("Directory <a> created"),
        Err(_why) => eprintln!("Directory <a> could not be created!"),
    }

    echo("hello",&Path::new("a/b.txt")).unwrap_or_else(|why|{
        eprintln!("Could not create: {:?}",why);
    });

    fs::create_dir_all("a/b/c/d.txt").unwrap_or_else(|why|{
        eprintln!("Could not create: {:?}",why);
    });

    touch(&Path::new("a/b/c/e.txt")).unwrap_or_else(|why|{
        eprintln!("Could not create: {:?}",why);
    });
#[cfg(target_family = "unix")] {
        unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }
    #[cfg(target_family = "windows")] {
        windows::fs::symlink_file("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
            println!("! {:?}", why.to_string());
        });
    }

    println!("`cat a/c/b.txt`");
    match cat(&Path::new("a/c/b.txt")) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(s) => println!("> {}", s),
    }

    match fs::read_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            println!("> {:?}", path.unwrap().path());
        },
    }

    fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    fs::remove_dir("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}
