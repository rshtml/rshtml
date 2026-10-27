use std::fs;
use std::io;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use std::sync::OnceLock;

pub fn create(path: &Path, generate: impl FnOnce() -> Vec<u8>) -> io::Result<PathBuf> {
    static MANIFEST_DIR: OnceLock<PathBuf> = OnceLock::new();
    let manifest_dir = match MANIFEST_DIR.get() {
        Some(dir) => dir,
        None => {
            let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
            let canonical = fs::canonicalize(&dir).map_err(|e| io::Error::new(e.kind(), format!("{dir:?}: {e}")))?;
            MANIFEST_DIR.get_or_init(|| canonical)
        }
    };

    let mut target_path = manifest_dir.join(".rshtml");

    let absolute_path = fs::canonicalize(manifest_dir.join(path))
        .map_err(|e| io::Error::new(e.kind(), format!("{}: {e}", path.display())))?;

    if let Ok(rel_path) = absolute_path.strip_prefix(manifest_dir) {
        target_path.push(rel_path);
    } else {
        const HEX: &[u8; 16] = b"0123456789abcdef";

        target_path.push("__ext__");

        for component in absolute_path.components() {
            match component {
                Component::Normal(c) => {
                    target_path.push(c);
                }
                Component::Prefix(prefix) => {
                    let bytes = prefix.as_os_str().as_encoded_bytes();
                    let mut hex = String::with_capacity(bytes.len() * 2 + 4);
                    hex.push_str("__");
                    for b in bytes {
                        hex.push(HEX[(b >> 4) as usize] as char);
                        hex.push(HEX[(b & 0xf) as usize] as char);
                    }
                    hex.push_str("__");

                    target_path.push(hex);
                }
                Component::CurDir | Component::ParentDir | Component::RootDir => {}
            }
        }
    }

    // .rs.html -> .rs
    if target_path.extension().is_some_and(|e| e == "html") {
        target_path.set_extension("");
    }

    if let (Ok(src_meta), Ok(tgt_meta)) = (fs::metadata(&absolute_path), fs::metadata(&target_path))
        && let (Ok(src_time), Ok(tgt_time)) = (src_meta.modified(), tgt_meta.modified())
        && src_time <= tgt_time
    {
        return Ok(target_path);
    }

    let generated_code = generate();

    if fs::read(&target_path).ok().as_deref() != Some(&generated_code) {
        let parent = target_path
            .parent()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "The file path is not valid"))?;
        fs::create_dir_all(parent)?;
        fs::write(&target_path, generated_code)?;
    }

    Ok(target_path)
}
