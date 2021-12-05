use std::path::*;

pub fn join<T>(parent: T, child: &str) -> PathBuf where PathBuf : From<T> {
    let mut result = PathBuf::from(parent);
    result.push(child);
    result
}
