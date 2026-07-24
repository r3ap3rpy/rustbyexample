#[cfg(target_os = "linux")]
fn only_on_linux() {
    println!("I am only available on linux!");
}

#[cfg(not(target_os = "linux"))]
fn only_not_on_linux() {
    println!("I am not available on linux!");
}

#[cfg(target_os = "macos")]
fn only_on_macos() {
    println!("I am runing on MacOS!");
}

fn main() {
    only_not_on_linux();
    only_on_macos();
    // To print all the built-in cfg features use the below command
    // rustc --print cfg
}
