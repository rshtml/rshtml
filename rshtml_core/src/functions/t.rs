use crate::functions::Functions;
use fluent::{FluentBundle, FluentResource};
use std::collections::HashMap;
use unic_langid::{LanguageIdentifier, langid};

impl Functions {
    pub fn t(&self, word: &str) -> String {
        todo!()
    }
}

fn load_translations() -> HashMap<LanguageIdentifier, FluentBundle<FluentResource>> {
    let mut bundles = HashMap::new();

    // İngilizce için
    let lang_en = langid!("en-US");
    let mut bundle_en = FluentBundle::new(vec![lang_en.clone()]);
    // Gerçekte dosya okuma olacak, fs::read_to_string("locales/en-US/main.ftl").unwrap() gibi
    let ftl_content_en = "hello = Hello World\ngoodbye = Good Bye".to_string();
    let resource_en = FluentResource::try_new(ftl_content_en).expect("EN FTL parse hatasi");
    bundle_en.add_resource(resource_en).expect("EN bundle ekleme hatasi");
    bundles.insert(lang_en, bundle_en);

    // Türkçe için
    let lang_tr = langid!("tr-TR");
    let mut bundle_tr = FluentBundle::new(vec![lang_tr.clone()]);
    let ftl_content_tr = "hello = Merhaba Dunya\ngoodbye = Hoscakal".to_string();
    let resource_tr = FluentResource::try_new(ftl_content_tr).expect("TR FTL parse hatasi");
    bundle_tr.add_resource(resource_tr).expect("TR bundle ekleme hatasi");
    bundles.insert(lang_tr, bundle_tr);

    bundles
}

struct Translator {
    bundles: HashMap<LanguageIdentifier, FluentBundle<FluentResource>>,
    current_lang: LanguageIdentifier,
}

impl Translator {
    fn new() -> Self {
        Self {
            bundles: load_translations(),
            current_lang: langid!("en-US"),
        }
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
