use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::path::*;

extern crate dosbox_lib;

use dosbox_lib::dosbox::run_dosbox;
use dosbox_lib::find::{find_dosbox, find_file_in_path};
use dosbox_lib::path_util::join;

fn main() -> Result<(), String> {
    match find_dosbox() {
        Some(dosbox) => run(dosbox),
        None => Err("Could not find DOSBox".to_string())
    }
}

fn run(dosbox: PathBuf) -> Result<(), String> {
    match find_file_in_path("QBASIC.EXE") {
        Some(qbasic) => run2(dosbox, qbasic),
        None => Err("Could not find QBASIC.EXE in PATH".to_string())
    }
}

fn run2(dosbox: PathBuf, qbasic: PathBuf) -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let bas_file = PathBuf::from(&args[0]);
    if bas_file.is_file() {
        run3(dosbox, qbasic, bas_file)
    } else {
        Err(format!("Could not find bas file {}", bas_file.display()))
    }
}

fn run3(dosbox: PathBuf, qbasic: PathBuf, bas_file: PathBuf) -> Result<(), String> {
    // copy qbasic into the same folder as the BAS_FILE
    let cwd = bas_file.parent().unwrap();
    let qbasic_copy = join(cwd, "QBASIC.EXE");
    copy_without_permissions(&qbasic, &qbasic_copy).unwrap();
    let cmd = format!("QBASIC.EXE /RUN {}", bas_file.file_name().unwrap().to_str().unwrap());
    run_dosbox(dosbox, cwd, &cmd).unwrap();
    fs::remove_file(qbasic_copy).unwrap();
    Ok(())
}

fn copy_without_permissions(src: &Path, dest: &Path) -> Result<u64, io::Error> {
    let mut src_file = File::open(src)?;
    let mut dest_file = File::create(dest)?;
    io::copy(&mut src_file, &mut dest_file)
}
