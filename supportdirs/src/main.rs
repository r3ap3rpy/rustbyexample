use std::fs::File;
use std::io::{self, IsTerminal, Write};
use std::env::current_exe;
use std::path::PathBuf;
use std::fs::create_dir_all;
use std::time::{UNIX_EPOCH,SystemTime};

#[derive(Debug)]
enum LogLevels {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

struct Logger {
    file: File,
    is_terminal: bool,
}

impl Logger {
    fn new(log_folder: &PathBuf, log_filename: &str) -> io::Result<Self> {
        let log_file = log_folder.join(log_filename);
        let file = File::options().create(true).append(true).open(log_file)?;
        Ok(Self {
            file,
            is_terminal: io::stdout().is_terminal(),
        })
    }
    fn log(&mut self, level: LogLevels, message: &str) -> io::Result<()> {
        let level = match level {
            LogLevels::Info => "INFO",
            LogLevels::Debug => "DEBUG",
            LogLevels::Warning => "WARNING",
            LogLevels::Error => "ERROR",
            LogLevels::Critical => "CRITICAL",
        };
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let message = format!("[{timestamp:^12}] :: [{level:^10}] -> {message}");
        writeln!(self.file,"{message}")?;
        if self.is_terminal {
            println!("{message}");
        }
        Ok(())
    }
    fn info(&mut self, message: &str) -> io::Result<()> {
        self.log(LogLevels::Info,message)
    }
    fn debug(&mut self,message: &str) -> io::Result<()> {
        self.log(LogLevels::Debug,message)
    }
    fn warning(&mut self,message: &str) -> io::Result<()> {
        self.log(LogLevels::Warning,message)
    }
    fn error(&mut self,message: &str) -> io::Result<()> {
        self.log(LogLevels::Error,message)
    }
    fn critical(&mut self,message: &str) -> io::Result<()> {
        self.log(LogLevels::Critical,message)
    }

}

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


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // choose this, because if current_dir is used and the cargo run command is ran
    // the relative location is the current folder where the cargo run was issued not where the
    // binary lives
    let executable_path = current_exe().expect("Cannot access binary!");
    let mut logfile_name = executable_path.file_name().unwrap().to_string_lossy().into_owned();
    logfile_name.push_str(".log");
    let folders = match verify_app_folder_state(&executable_path) {
        Ok(folders) => folders,
        Err(e) => panic!("Folder state is incomplete: {}",e)
    };
    println!("Executable: {:?}",executable_path);
    println!("The configuration folder: {:?}",folders.config);
    println!("The logs folder: {:?}",folders.logs);
    let mut app_logger = Logger::new(&folders.logs, &logfile_name)?;
    app_logger.info("This is an informational level message!")?;
    std::thread::sleep(std::time::Duration::from_secs(5));
    app_logger.debug("5 seconds later")?;
    app_logger.warning("5 seconds later")?;
    app_logger.error("5 seconds later")?;
    app_logger.critical("5 seconds later")?;
     Ok(())
}
