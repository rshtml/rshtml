use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Config {
    pub views_base_path: PathBuf,
}

#[allow(dead_code)]
impl Config {
    pub fn new<P: AsRef<Path>>(views_base_path: P) -> Self {
        Config {
            views_base_path: views_base_path.as_ref().to_path_buf(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        let mut base_path = PathBuf::from(manifest_dir);
        base_path.push("src");
        base_path.push("views");
        Config { views_base_path: base_path }
    }
}
