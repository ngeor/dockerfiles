use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::*;
use std::process::Command;

use path_util::join;

pub fn run_dosbox(dosbox: PathBuf, cwd: &Path, command: &str) -> Result<(), String> {
    if !cwd.is_dir() {
        return Err(format!("Could not find run directory {}", cwd.display()));
    }

    if command.is_empty() {
        return Err("Command not given".to_string());
    }

    let stdout_file_name = "OUT.TXT";
    let stdout_file = join(cwd, stdout_file_name);
    let batch_file = join(cwd, "WRAP.BAT");
    create_batch_wrapper(stdout_file_name, &batch_file, command).unwrap();

    let dosbox_conf = join(cwd, "dosbox.conf");
    create_minimal_dosbox_config(&dosbox_conf).unwrap();

    let result = Command::new(dosbox)
        .args(&[batch_file.to_str().unwrap(), "-exit", "-noconsole", "-noautoexec", "-conf", dosbox_conf.to_str().unwrap()])
        .env_clear()
        .env("SDL_VIDEODRIVER", "dummy")
        .env("TERM", "dumb")
        .output()
        .expect("Error running DOSBox");
    if !result.status.success() {
        return Err("DOSBox did not return a success error code".to_string());
    }
    let out = fs::read(&stdout_file).unwrap();
    io::stdout().write_all(&out).unwrap();
    fs::remove_file(batch_file).unwrap();
    fs::remove_file(stdout_file).unwrap();
    fs::remove_file(dosbox_conf).unwrap();
    Ok(())
}

fn create_batch_wrapper(stdout_file_name: &str, batch_file: &PathBuf, cmd: &str) -> Result<(), std::io::Error> {
    let mut f = File::create(batch_file)?;
    write!(f, "@ECHO OFF\r\n")?;
    // switch to C: drive
    write!(f, "C:\r\n")?;
    write!(f, "{} > {}\r\n", cmd, stdout_file_name)?;
    Ok(())
}

fn create_minimal_dosbox_config(p: &PathBuf) -> Result<(), std::io::Error> {
    let mut f = File::create(p)?;
    write!(f, r"[cpu]
cycles = max
core = dynamic

[midi]
mpu401 = none
mididevice = none
")?;
    Ok(())
}
