use std::fs;
use std::path::Path;

fn main() {
    let views_dir = Path::new("src/views");

    if views_dir.is_dir() {
        walk_dir(views_dir);
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
