pub use rshtml_core::functions;
pub use rshtml_core::traits;
pub use rshtml_macro::RsHtml;

use rshtml_core::config;
use std::fs;
use std::path::Path;

pub fn track_views_folder() {
    let config = config::Config::load_from_toml_or_default();

    if config.views.0.is_dir() {
        walk_dir(&config.views.0);
    }
}

fn walk_dir(dir: &Path) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    walk_dir(&path);
                } else if path.is_file() {
                    if let Some(path_str) = path.to_str() {
                        println!("cargo:rerun-if-changed={}", path_str);
                    }
                }
            }
        }
    }
}
