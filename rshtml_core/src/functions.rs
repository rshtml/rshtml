mod t;
mod time;

use crate::functions::t::Translator;
use serde::Serialize;
use std::string::ToString;
use std::sync::RwLock;

pub struct Functions {
    pub layout: String,
    sections: Vec<String>,
    pushed_texts: Vec<String>,
    translator: RwLock<Translator>,
}

impl Functions {
    pub fn new(layout: String, sections: Vec<String>, locales_base_path: &str, locale_lang: &str) -> Self {
        Self {
            layout,
            sections,
            pushed_texts: Vec::new(),
            translator: RwLock::new(Translator::new(locales_base_path, locale_lang)),
        }
    }

    pub fn has_section(&self, section_name: &str) -> bool {
        self.sections.contains(&section_name.to_string())
    }

    pub fn json<T: Serialize>(&self, value: &T) -> String {
        serde_json::to_string(value).unwrap_or_else(|err| {
            eprintln!("DEBUG: JSON error: {}", err);
            "{}".to_string()
        })
    }

    pub fn json_let<T: Serialize>(&self, name: &str, value: &T) -> String {
        let json = serde_json::to_string(value).unwrap_or_else(|err| {
            eprintln!("DEBUG: JSON error: {}", err);
            "{}".to_string()
        });

        format!("let {} = {}", name, json)
    }

    pub fn class(&self, classes: &[&str]) -> String {
        format!("class=\"{}\"", classes.join(" "))
    }

    pub fn css(&self, css: &[(&str, &str)]) -> String {
        format!("style=\"{}\"", css.iter().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<_>>().join("; "))
    }

    pub fn push(&mut self, text: &str) {
        self.pushed_texts.push(text.to_string());
    }

    pub fn stack(&self) -> String {
        self.pushed_texts.iter().fold(String::new(), |mut x, y| {
            x.push_str(y);
            x.push('\n');
            x
        })
    }
}
