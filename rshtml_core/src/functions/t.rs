use crate::functions::Functions;
use anyhow::{Result, anyhow};
use fluent::FluentResource;
use fluent::concurrent::FluentBundle;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use unic_langid::{LanguageIdentifier, langid};

impl Functions {
    pub fn t(&self, word: &str) -> String {
        self.translator.translate(word)
    }
}

pub struct Translator {
    bundles: HashMap<LanguageIdentifier, FluentBundle<FluentResource>>,
    current_lang: LanguageIdentifier,
}

impl Translator {
    pub fn new(locales_base_path: &str) -> Self {
        let bundles = Self::load(Path::new(locales_base_path)).unwrap_or_else(|err| {
            eprintln!("Locale Error: {}", err);
            HashMap::new()
        });

        Self {
            bundles,
            current_lang: langid!("en-US"),
        }
    }

    pub fn load(locales_path: &Path) -> Result<HashMap<LanguageIdentifier, FluentBundle<FluentResource>>> {
        let mut bundles = HashMap::new();

        if !locales_path.exists() || !locales_path.is_dir() {
            return Err(anyhow!("Locales path doesn't exist: {:?}", locales_path));
        }

        for dir in fs::read_dir(locales_path)? {
            let lang_path = dir?.path();

            if !lang_path.is_dir() {
                continue;
            }

            let Some(lang_code_str) = lang_path.file_name().and_then(|x| x.to_str()) else {
                continue;
            };

            let lang_id = lang_code_str.parse::<LanguageIdentifier>()?;

            let mut bundle = FluentBundle::new_concurrent(vec![lang_id.clone()]);
            let mut ftl_files_found = false;

            for file in fs::read_dir(&lang_path)? {
                let ftl_file_path = file?.path();

                if !ftl_file_path.is_file() || !ftl_file_path.extension().map_or(false, |ext| ext == "ftl") {
                    continue;
                }

                let ftl_content = fs::read_to_string(&ftl_file_path)?;
                let resource = FluentResource::try_new(ftl_content).map_err(|(_, errs)| anyhow!("{:?}: {:?}", ftl_file_path, errs))?;

                bundle.add_resource(resource).map_err(|errs| anyhow!("{:?}: {:?}", ftl_file_path, errs))?;

                ftl_files_found = true;
            }

            if ftl_files_found {
                bundles.insert(lang_id, bundle);
            }
        }

        Ok(bundles)
    }

    pub fn translate(&self, key: &str) -> String {
        // Aktif dilin çeviri paketini al
        if let Some(bundle) = self.bundles.get(&self.current_lang) {
            // Mesajı al
            if let Some(message) = bundle.get_message(key) {
                // Mesajın değerini (çevrilmiş metni) al
                if let Some(pattern) = message.value() {
                    let mut errors = Vec::new();
                    // format_pattern parametresiz mesajlar için None args alır
                    let translated_text = bundle.format_pattern(pattern, None, &mut errors);
                    if errors.is_empty() {
                        return translated_text.into_owned(); // Cow den String e
                    } else {
                        // Formatlama hatası oldu
                        return format!("FORMAT_ERROR:{}", key);
                    }
                }
            }
        }
        // Çeviri bulunamadı veya hata oldu
        format!("MISSING:{}", key)
    }
}
