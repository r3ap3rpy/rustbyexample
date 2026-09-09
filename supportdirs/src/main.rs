use std::env::{current_exe};
use std::path::{PathBuf};
use std::fs::create_dir_all;

#[derive(Debug)]
struct Folders {
    logs: PathBuf,
    config: PathBuf,
}

fn verify_app_folder_state(exe_path: &PathBuf) -> Result<Folders,Box<dyn std::error::Error>> {
    let app_dir = exe_path.parent().expect("Cannot reach parent folder!");
    let config_dir = app_dir.join("config");
    let logs_dir = app_dir.join("logs");
    create_dir_all(&config_dir)?;
    create_dir_all(&logs_dir)?;
    Ok(Folders { logs: logs_dir, config: config_dir })
}


fn main() {
    // choose this, because if current_dir is used and the cargo run command is ran
    // the relative location is the current folder where the cargo run was issued not where the
    // binary lives
    let executable_path = current_exe().expect("Cannot access binary!");

    let folders = match verify_app_folder_state(&executable_path) {
        Ok(folders) => folders,
        Err(e) => panic!("Folder state is incomplete: {}",e)
    };
    println!("Executable: {:?}",executable_path);
    println!("The configuration folder: {:?}",folders.config);
    println!("The logs folder: {:?}",folders.logs);
}
