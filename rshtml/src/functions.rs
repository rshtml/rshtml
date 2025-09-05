mod time;

use serde::Serialize;
use std::string::ToString;
pub use time::*;

pub fn json<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value).unwrap_or_else(|err| {
        eprintln!("DEBUG: JSON error: {err}");
        "{}".to_string()
    })
}

pub fn json_let<T: Serialize>(name: &str, value: &T) -> String {
    let json = serde_json::to_string(value).unwrap_or_else(|err| {
        eprintln!("DEBUG: JSON error: {err}");
        "{}".to_string()
    });

    format!("let {name} = {json}")
}
