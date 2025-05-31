mod t;
mod time;

use serde::Serialize;
use std::string::ToString;

pub struct Functions;

impl Functions {

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
}
