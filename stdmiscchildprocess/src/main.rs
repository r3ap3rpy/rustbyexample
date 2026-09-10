use std::process::Command;

fn main() {
    let output = Command::new("rustc").arg("--version").output()
        .unwrap_or_else(|e|{
            panic!("Command failed with: {:?}",e);
        });
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("The output was: {:?}",s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        eprintln!("The error was: {:?}",s);
    }
}
