use std::fs;
use std::io;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

pub fn create(path: &Path, generated_code: &str) -> io::Result<PathBuf> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());

    let mut target_path = PathBuf::from(&manifest_dir);
    target_path.push(".rshtml");

    for component in path.components() {
        match component {
            Component::Normal(c) => {
                target_path.push(c);
            }
            Component::ParentDir => {
                target_path.push("__up__");
            }
            Component::RootDir => {
                target_path.push("__root__");
            }
            Component::Prefix(prefix) => {
                let hex: String = prefix
                    .as_os_str()
                    .to_string_lossy()
                    .bytes()
                    .map(|b| format!("{:02x}", b))
                    .collect();
                target_path.push(format!("__{}__", hex));
            }
            Component::CurDir => {}
        }
    }

    // .rs.html -> .rs
    if target_path.extension().is_some_and(|e| e == "html") {
        target_path.set_extension("");
    }

    let parent = target_path
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "The file path is not valid"))?;
    fs::create_dir_all(parent)?;

    if fs::read(&target_path).ok().as_deref() != Some(generated_code.as_bytes()) {
        fs::write(&target_path, generated_code)?;
    }

    Ok(target_path)
}
