//! [![Crates.io Version](https://img.shields.io/crates/v/rshtml.svg)](https://crates.io/crates/rshtml)
//! [![GitHub Repository](https://img.shields.io/badge/github-repo-blue.svg?logo=github)](https://github.com/rshtml/rshtml)
//! [![Docs.rs Documentation](https://docs.rs/rshtml/badge.svg)](https://docs.rs/rshtml)
//! [![Full Documentation](https://img.shields.io/badge/book-rshtml.github.io-blue.svg)](https://rshtml.github.io/)
//!
//! # RsHtml: A Template Engine for Seamless HTML and Rust Integration.
//!
//! RsHtml is a powerful template engine that transforms your HTML templates
//! into highly efficient Rust code at compile time, allowing you to seamlessly use
//! Rust logic and expressions together with HTML to harness the full power of Rust
//! for dynamic content generation. It is designed to help you build flexible and
//! maintainable web applications.
//!
//! ![Demo](https://raw.githubusercontent.com/rshtml/rshtml/master/v_macro.gif)
//!
//! ## Quick Start
//!
//! **1. Add to `Cargo.toml`:**
//!
//! ```toml
//! [dependencies]
//! rshtml = "0.5.0" # Use the latest version
//! ```
//! ```rust
//! use rshtml::{traits::View, v};
//! use std::fmt;
//!
//! fn main() -> fmt::Result {
//!   let template = "RsHtml";
//!   let hello = v!(<p>Hello {template}</p>);
//!
//!   let mut out = String::with_capacity(hello.text_size());
//!
//!   hello.render(&mut out)?;
//!  
//!   print!("{out}");
//!
//!   Ok(())
//! }
//! ```

/// Utility functions for use directly in RsHtml templates.
///
/// Example template usage: `@time(&self.my_date)`, `@json(&self.data)`.
#[cfg(feature = "functions")]
pub mod functions;
pub mod traits;

mod escaping_writer;
pub use escaping_writer::EscapingWriter;

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

/// Instructs Cargo to recompile the crate if any file in the views folder changes.
///
/// This function should be called from a `build.rs` script.
/// It helps ensure that template changes are picked up during development
/// without needing a full manual recompile of the dependent crate.
mod track_views_folder;
pub use track_views_folder::track_views_folder;

mod expr;
pub use expr::{Block, Expr};

mod exp;
pub use exp::Exp;
mod view_fn;
pub use view_fn::ViewFn;
mod text_size;
pub use text_size::TextSize;
mod view_iter;
pub use view_iter::ViewIter;

pub use rshtml_macro::v;
pub use rshtml_macro::v_file;
