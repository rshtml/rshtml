use toml::Value;

use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub fn track_views_folder() {
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let cargo_toml_path = Path::new(&manifest_dir).join("Cargo.toml");
        if let Ok(content) = std::fs::read_to_string(cargo_toml_path)
            && let Ok(manifest) = toml::from_str::<Value>(&content)
            && let Some(pkg) = manifest.get("package")
            && let Some(metadata) = pkg.get("metadata")
            && let Some(toml_config) = metadata.get("rshtml")
            && let Some(views) = toml_config.get("views")
            && let Some(views_path) = views.get("path")
            && let Some(views_path_str) = views_path.as_str()
        {
            let mut base_path = PathBuf::from(&manifest_dir);
            base_path.push(views_path_str);

            if base_path.is_dir() {
                walk_dir(&base_path);
            }
        }
    }
}

fn walk_dir(dir: &Path) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk_dir(&path);
            } else if path.is_file()
                && let Some(path_str) = path.to_str()
            {
                println!("cargo:rerun-if-changed={path_str}");
            }
        }
    }
}
