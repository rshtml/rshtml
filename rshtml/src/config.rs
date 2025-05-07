use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub views_base_path: PathBuf,
    pub layout: String,
}

#[allow(dead_code)]
impl Config {
    pub fn new<P: AsRef<Path>>(views_base_path: P, layout: String) -> Self {
        Config {
            views_base_path: views_base_path.as_ref().to_path_buf(),
            layout,
        }
    }

    pub fn set_views_base_path(&mut self, path_str: String) -> &mut Self {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        let mut base_path = PathBuf::from(manifest_dir);
        base_path.push(path_str);
        self.views_base_path = base_path;

        self
    }

    pub fn set_layout(&mut self, layout_str: String) -> &mut Self {
        self.layout = layout_str;

        self
    }
}

impl Default for Config {
    fn default() -> Self {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        let mut base_path = PathBuf::from(manifest_dir);
        base_path.push("src");
        base_path.push("views");
        Config {
            views_base_path: base_path,
            layout: String::from("layout.rs.html"),
        }
    }
}
