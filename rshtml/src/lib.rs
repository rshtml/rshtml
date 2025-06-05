//! # RsHtml: A Template Engine for Seamless HTML and Rust Integration.
//!
//! RsHtml is a templating engine for Rust that allows you to embed Rust logic
//! directly into HTML-like template files. It uses a `#[derive(RsHtml)]` macro
//! to process these templates at compile time, generating efficient Rust code
//! for rendering.
//!
//! ## Core Idea
//!
//! 1.  **Define a Rust struct** (e.g., `HomePage`) to hold your template's data.
//! 2.  **Annotate it with `#[derive(RsHtml)]`**.
//! 3.  **Create a template file** (e.g., `home.rs.html`) using `@` prefixed
//!     expressions, control flow (`@if`, `@for`), components, layouts, etc.
//! 4.  **Call the generated `render()` method** on an instance of your struct and get the HTML output.
//!
//! ## Key Syntax Highlights
//!
//! - **Expressions:** `@self.field`, `@self.my_function()`, `@(3 + 5)`
//! - **Control Flow:** `@if ...`, `@for ...`, `@match ...`
//! - **Code Blocks:** `@{ let a = 5; }`
//! - **Layouts/Sections:** `@extends`, `@section`, `@render`
//! - **Components:** `<MyComponent data=@value />` or `@MyComponent { ... }`
//! - **Raw Blocks:** `@raw{ ... }`
//! - **Comments:** `@* server-side comment *@`
//!
//! (For a detailed syntax reference, see the project's main documentation.)
//!
//! ## Quick Start
//!
//! **1. Add to `Cargo.toml`:**
//!
//! ```toml
//! [dependencies]
//! rshtml = "0.1.0" # Use the latest version
//! ```
//!
//! **2. Define Struct and Template:**
//!
//! ```rust
//! // src/main.rs
//! use rshtml::RsHtml;
//!
//! #[derive(RsHtml)] // Looks for "home.rs.html" by default (adjust to your naming)
//! // #[rshtml(path = "home.rs.html")]
//! struct HomePage {
//!     product_name: String,
//!     price: f64,
//! }
//!
//! fn main() {
//!     let home = HomePage {
//!         product_name: "Ferris".to_string(),
//!         price: 99.99,
//!     };
//!
//!     // Assumes the derive macro generates a `render()` method.
//!     let html_output = home.render().unwrap();
//!     println!("{}", html_output);
//! }
//! ```
//!
//! ```html
//!  Template file (e.g., home.rs.html):
//!  <div>
//!    <h2>@self.product_name</h2>
//!    <p>Price: $@self.price</p>
//!    @if self.price < 100.0 {
//!      <span>Special Offer!</span>
//!    }
//!  </div>
//! ```
//!
//! **3. Set build.rs file for tracking view changes on build:** (Optional)
//!
//! ```rust
//! use rshtml::track_views_folder;
//!
//! fn main() {
//!     track_views_folder();
//! }
//! ```
//!
//!  **4. Change views settings:** (Optional)
//! ```toml
//! [package.metadata.rshtml]
//! views = { path = "views", layout = "layout.rs.html" } # these are the default values
//! ```
//!
//!
//! ## The `RsHtml` Derive Macro
//!
//! - **[`RsHtml` (derive macro)]**: The main entry point. Apply to a struct to
//!   enable template rendering. It handles parsing the associated template file
//!   (path can be customized via `#[rshtml(path = "...")]`) and generates
//!   the rendering logic.
//!
//! ---
//!
//! *For more examples and detailed information on all directives and features,
//! please consult the project's repository or extended documentation.*

/// Utility functions for use directly in RsHtml templates.
///
/// Example template usage: `@time(&self.my_date)`, `@json(&self.data)`.
pub use rshtml_core::functions;
pub use rshtml_core::traits;

/// The primary derive macro for enabling RsHtml templating on a struct.
///
/// Apply `#[derive(RsHtml)]` to a Rust struct to associate it with an
/// HTML-like template file. This macro processes the template at compile time,
/// generating the necessary Rust code to render it based on the struct's fields
/// and methods.
///
/// By default, the macro attempts to find a template file named after the
/// struct (e.g., `HomePage` struct maps to `home.rs.html`).
/// This path can be customized using the `#[rshtml(path = "custom.rs.html")]` attribute
/// on the struct.
///
/// Once derived, an instance of the struct will have a `render()` method to produce the HTML output.
pub use rshtml_macro::RsHtml;

use rshtml_core::config;
use std::fs;
use std::path::Path;

/// Instructs Cargo to recompile the crate if any file in the views folder changes.
///
/// This function should be called from a `build.rs` script.
/// It helps ensure that template changes are picked up during development
/// without needing a full manual recompile of the dependent crate.
pub fn track_views_folder() {
    let config = config::Config::load_from_toml_or_default();

    if config.views.0.is_dir() {
        walk_dir(&config.views.0);
    }
}

fn walk_dir(dir: &Path) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    walk_dir(&path);
                } else if path.is_file() {
                    if let Some(path_str) = path.to_str() {
                        println!("cargo:rerun-if-changed={}", path_str);
                    }
                }
            }
        }
    }
}
