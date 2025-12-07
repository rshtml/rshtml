use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug, Clone)]
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
        #[derive(Deserialize, Debug, Clone)]
        pub struct Views {
            pub path: Option<String>,
            pub extract_file_on_debug: Option<bool>,
        }

        #[derive(Deserialize, Debug, Clone)]
        pub struct MetadataConfig {
            pub views: Option<Views>,
        }

        #[derive(Deserialize, Debug)]
        struct Metadata {
            rshtml: Option<MetadataConfig>,
        }

        #[derive(Deserialize, Debug)]
        struct Package {
            metadata: Option<Metadata>,
        }

        #[derive(Deserialize, Debug)]
        struct Manifest {
            package: Option<Package>,
        }

        let mut config = Self::default();

        if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
            let cargo_toml_path = Path::new(&manifest_dir).join("Cargo.toml");
            if let Ok(content) = std::fs::read_to_string(cargo_toml_path)
                && let Ok(manifest) = toml::from_str::<Manifest>(&content)
                && let Some(pkg) = manifest.package
                && let Some(metadata) = pkg.metadata
                && let Some(toml_config) = metadata.rshtml
                && let Some(views) = toml_config.views
            {
                config.set_views(views.path, views.extract_file_on_debug);
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
