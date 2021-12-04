use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;

pub fn make_unique_random_filename(parent: &Path, extension: &str, rand: &mut Rand) -> PathBuf {
    loop {
        let mut result: PathBuf = parent.to_path_buf();
        let mut filename: String = make_random_filename(rand);
        filename.push('.');
        filename.push_str(extension);
        result.push(filename);
        if !result.exists() {
            return result;
        }
    }
}

fn make_random_filename(rand: &mut Rand) -> String {
    let letters: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut result: String = String::new();
    for _ in 0..8 {
        let letter_index: usize = (rand.next() % (letters.len() as u128)) as usize;
        result.push(letters[letter_index]);
    }
    return result;
}

pub struct Rand {
    state: u128
}

impl Rand {
    pub fn new() -> Self {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => {
                let millis = duration.as_millis();
                return Self { state: millis }
            }
            Err(_) => {
                panic!("SystemTime before UNIX_EPOCH!")
            }
        }
    }

    pub fn next(&mut self) -> u128 {
        self.state = (self.state * 125) % 2796203;
        self.state
    }
}
