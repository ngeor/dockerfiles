use std::collections::hash_map::HashMap;
use std::env;
use std::path::*;

extern crate dosbox_lib;

use dosbox_lib::find::find_dosbox;
use dosbox_lib::dosbox::run_dosbox;

fn main() -> Result<(), String> {
    match find_dosbox() {
        Some(dosbox) => run(dosbox),
        None => Err("Could not find DOSBox".to_string())
    }
}

fn run(dosbox: PathBuf) -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let cwd = PathBuf::from(&args[0]);
    let command = &args[1];
    run_dosbox(dosbox, &cwd, command, &HashMap::new())
}
