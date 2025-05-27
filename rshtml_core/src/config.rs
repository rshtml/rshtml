use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub views: (PathBuf, String),   // base_path, layout
    pub locales: (PathBuf, String), // base_path, default_lang
}

#[allow(dead_code)]
impl Config {
    pub fn new<P: AsRef<Path>>(views: (PathBuf, String), locales: (PathBuf, String)) -> Self {
        Config { views, locales }
    }

    pub fn set_views(&mut self, views: (String, String)) {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());

        let mut base_path = PathBuf::from(&manifest_dir);
        base_path.push(views.0);
        self.views = (base_path, views.1);
    }

    pub fn set_locales(&mut self, locales: (String, String)) {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());

        let mut base_path = PathBuf::from(&manifest_dir);
        base_path.push(locales.0);
        self.locales = (base_path, locales.1);
    }

    pub fn load_from_toml_or_default() -> Self {
        #[derive(Deserialize, Debug, Clone)]
        pub struct Views {
            pub path: String,
            pub layout: String,
        }

        #[derive(Deserialize, Debug, Clone)]
        pub struct Locales {
            pub path: String,
            pub lang: String,
        }

        #[derive(Deserialize, Debug, Clone)]
        pub struct MetadataConfig {
            pub views: Option<Views>,
            pub locales: Option<Locales>,
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
            if let Ok(content) = std::fs::read_to_string(cargo_toml_path) {
                match toml::from_str::<Manifest>(&content) {
                    Ok(manifest) => {
                        if let Some(pkg) = manifest.package {
                            if let Some(metadata) = pkg.metadata {
                                if let Some(toml_config) = metadata.rshtml {
                                    if let Some(views) = toml_config.views {
                                        config.set_views((views.path, views.layout));
                                    }
                                    if let Some(locales) = toml_config.locales {
                                        config.set_locales((locales.path, locales.lang));
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => {}
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
        let mut locales_base_path = base_path.clone();
        locales_base_path.push("locales");

        Config {
            views: (views_base_path.clone(), String::from("layout.rs.html")),
            locales: (locales_base_path.clone(), String::from("en-US")),
        }
    }
}
