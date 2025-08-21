use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub views: (PathBuf, String), // base_path, layout
}

#[allow(dead_code)]
impl Config {
    pub fn new<P: AsRef<Path>>(views: (PathBuf, String)) -> Self {
        Config { views }
    }

    pub fn set_views(&mut self, views: (String, String)) {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());

        let mut base_path = PathBuf::from(&manifest_dir);
        base_path.push(views.0);
        self.views = (base_path, views.1);
    }

    pub fn load_from_toml_or_default() -> Self {
        #[derive(Deserialize, Debug, Clone)]
        pub struct Views {
            pub path: String,
            pub layout: String,
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
                config.set_views((views.path, views.layout));
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
        let mut locales_base_path = base_path.clone();
        locales_base_path.push("locales");

        Config {
            views: (views_base_path.clone(), String::from("layout.rs.html")),
        }
    }
}
