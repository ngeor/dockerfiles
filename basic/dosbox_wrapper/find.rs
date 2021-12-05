use std::env;
use std::path::*;

use path_util::join;

pub fn find_dosbox() -> Option<PathBuf> {
    let is_windows = "windows".eq_ignore_ascii_case(env::consts::OS);
    let dosbox_in_path = find_file_in_path(
        if is_windows { "DOSBox.exe" } else { "dosbox" }
    );
    if dosbox_in_path.is_none() && is_windows {
        find_dosbox_in_program_files()
    } else {
        dosbox_in_path
    }
}

pub fn find_file_in_path(filename: &str) -> Option<PathBuf> {
    let key = "PATH";
    match env::var_os(key) {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let candidate = join(&path, filename);
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
        }
        None => {
            eprintln!("{} is not defined in the environment.", key)
        }
    }
    None
}

fn find_dosbox_in_program_files() -> Option<PathBuf> {
    let p = PathBuf::from("C:\\Program Files (x86)\\DOSBox-0.74\\DOSBox.exe");
    if p.is_file() {
        Some(p)
    } else {
        None
    }
}
