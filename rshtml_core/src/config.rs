use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Config {
    pub base_path: PathBuf,
    pub extract_file_on_debug: bool,
}

impl Config {
    pub fn new<P: AsRef<Path>>(base_path: PathBuf, extract_file_on_debug: bool) -> Self {
        Config {
            base_path,
            extract_file_on_debug,
        }
    }

    pub fn set_views(&mut self, path: Option<String>, extract_file_on_debug: Option<bool>) {
        if let Some(p) = path {
            let manifest_dir =
                std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());

            let mut base_path = PathBuf::from(&manifest_dir);
            base_path.push(p);
            self.base_path = base_path;
        }

        if let Some(ef) = extract_file_on_debug {
            self.extract_file_on_debug = ef;
        }
    }

    pub fn load_from_toml_or_default() -> Self {
        let mut config = Self::default();

        if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
            let cargo_toml_path = Path::new(&manifest_dir).join("Cargo.toml");
            if let Ok(content) = std::fs::read_to_string(cargo_toml_path)
                && let Ok(toml_value) = content.parse::<toml::Value>()
            {
                let views = toml_value
                    .get("package")
                    .and_then(|v| v.get("metadata"))
                    .and_then(|v| v.get("rshtml"))
                    .and_then(|v| v.get("views"));

                if let Some(v) = views {
                    let path = v
                        .get("path")
                        .and_then(|p| p.as_str())
                        .map(|s| s.to_string());
                    let extract_file_on_debug =
                        v.get("extract_file_on_debug").and_then(|e| e.as_bool());

                    config.set_views(path, extract_file_on_debug);
                }
            }
        }

        config
    }
}

impl Default for Config {
    fn default() -> Self {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        let base_path = PathBuf::from(manifest_dir);
        let mut views_base_path = base_path.clone();
        views_base_path.push("views");

        Config {
            base_path: views_base_path.clone(),
            extract_file_on_debug: false,
        }
    }
}
