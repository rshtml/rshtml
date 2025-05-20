use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub views_base_path: PathBuf,
    pub layout: String,
    pub locales_base_path: PathBuf,
    pub locale_lang: String,
}

#[allow(dead_code)]
impl Config {
    pub fn new<P: AsRef<Path>>(views_base_path: P, layout: String, locales_base_path: P, locale_lang: String) -> Self {
        Config {
            views_base_path: views_base_path.as_ref().to_path_buf(),
            layout,
            locales_base_path: locales_base_path.as_ref().to_path_buf(),
            locale_lang,
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

    pub fn set_locales_base_path(&mut self, locale_str: String) -> &mut Self {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        let mut base_path = PathBuf::from(manifest_dir);
        base_path.push(locale_str);
        self.locales_base_path = base_path;

        self
    }

    pub fn set_locale_lang(&mut self, locale_lang: String) -> &mut Self {
        self.locale_lang = locale_lang;

        self
    }

    pub fn load_from_toml_or_default() -> Self {
        #[derive(Deserialize, Debug, Clone)]
        pub struct MetadataConfig {
            pub views_base_path: Option<String>,
            pub layout: Option<String>,
            pub locales_base_path: Option<String>,
            pub locale_lang: Option<String>,
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
                                    if let Some(path_str) = toml_config.views_base_path {
                                        config.set_views_base_path(path_str);
                                    }
                                    if let Some(layout_str) = toml_config.layout {
                                        config.set_layout(layout_str);
                                    }
                                    if let Some(locale_str) = toml_config.locales_base_path {
                                        config.set_locales_base_path(locale_str);
                                    }
                                    if let Some(locale_lang_str) = toml_config.locale_lang {
                                        config.set_locale_lang(locale_lang_str);
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
            views_base_path,
            layout: String::from("layout.rs.html"),
            locales_base_path,
            locale_lang: String::from("en-US"),
        }
    }
}
