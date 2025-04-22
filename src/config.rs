use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Config {
    pub views_base_path: PathBuf,
}

impl Config {
    pub fn new<P: AsRef<Path>>(views_base_path: P) -> Self {
        Config {
            views_base_path: views_base_path.as_ref().to_path_buf(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            views_base_path: PathBuf::from("src/views"),
        }
    }
}