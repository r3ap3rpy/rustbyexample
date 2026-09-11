use std::io::{Read, Write};
use std::process::{Command, Stdio};

static PANAGRAM: &'static str = "the quick brown fox jumps over the lazy dog\n";

fn main() {
    let mut cmd = if cfg!(target_family = "windows") {
        let mut cmd = Command::new("powershell");
        cmd.arg("-Command").arg("$input | Measure-Object -Line -Word -Character");
        cmd
    } else {
        Command::new("wc")
    };
    let process = match cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn() {
        Err(why) => panic!("Could not open process because: {:?}",why),
        Ok(process) => process,
    };
    match process.stdin.unwrap().write_all(PANAGRAM.as_bytes()) {
        Err(why) => panic!("could not write to wc because: {:?}",why),
        Ok(_) => println!("sent input to wc"),
    }
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("Could not read output because: {:?}",why),
        Ok(_) => println!("response was: {:?}",s),
    }
}
