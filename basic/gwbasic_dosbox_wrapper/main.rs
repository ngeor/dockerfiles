use std::collections::hash_map::HashMap;
use std::env;
use std::fs::{self, File};
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
    match find_file_in_path("GWBASIC.EXE") {
        Some(gwbasic) => run2(dosbox, gwbasic),
        None => Err("Could not find GWBASIC.EXE in PATH".to_string())
    }
}

fn run2(dosbox: PathBuf, gwbasic: PathBuf) -> Result<(), String> {
    match find_bas_file() {
        Ok(bas_file) => run3(dosbox, gwbasic, bas_file),
        Err(err) => Err(err)
    }
}

fn find_bas_file() -> Result<PathBuf, String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        Err("Please specify the BAS file to run".to_string())
    } else {
        let bas_file = PathBuf::from(&args[0]);
        if bas_file.is_file() {
            Ok(bas_file)
        } else {
            Err(format!("Could not find BAS file {}", bas_file.display()))
        }
    }
}

fn run3(dosbox: PathBuf, gwbasic: PathBuf, bas_file: PathBuf) -> Result<(), String> {
    // copy GWBASIC into the same folder as the BAS_FILE
    let cwd = bas_file.parent().unwrap();
    let gwbasic_copy = join(cwd, "GWBASIC.EXE");
    copy_without_permissions(&gwbasic, &gwbasic_copy).unwrap();
    let cmd = format!("GWBASIC.EXE {}", bas_file.file_name().unwrap().to_str().unwrap());
    run_dosbox(dosbox, cwd, &cmd, &HashMap::new()).unwrap();
    fs::remove_file(gwbasic_copy).unwrap();
    Ok(())
}

fn copy_without_permissions(src: &Path, dest: &Path) -> Result<u64, io::Error> {
    let mut src_file = File::open(src)?;
    let mut dest_file = File::create(dest)?;
    io::copy(&mut src_file, &mut dest_file)
}
