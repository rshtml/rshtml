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
//! **Add to `Cargo.toml`:**
//!
//! ```toml
//! [dependencies]
//! rshtml = "0.6.0" # Use the latest version
//! ```
//!
//! **v! macro, to write HTML within Rust:**
//! ```rust
//! use rshtml::{View, v};
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
//!
//! **For the `.rs.html` file:**
//!
//! ```rust
//! use rshtml::View;
//!
//! // The `path` parameter specifies a path relative to `CARGO_MANIFEST_DIR` or the
//! // current working directory. The `extract` parameter extracts the Rust segments from
//! // the `.rs.html` file into a separate file and includes it, thereby simplifying error handling.
//! #[derive(View)]
//! // #[view(path = "views/home.rs.html", extract)] // These are optional.
//! struct HomePage { // Looks for views/home.rs.html in views folder.
//!     title: String,
//! }
//!
//!
//! fn main() {
//!    let homepage = HomePage {
//!        title: "Home Page".to_string()
//!    };
//!
//!     let mut out = String::with_capacity(homepage.text_size());
//!     homepage.render(&mut out).unwrap();
//!
//!     print!("{}", out);
//! }
//! ```
//!
//! The `View` macro implements the `View` trait for the struct, making it usable with the `v!` macro.

/// Utility functions for use directly in RsHtml templates.
///
/// Example template usage: `@time(&self.my_date)`, `@json(&self.data)`.
#[cfg(feature = "functions")]
pub mod functions;

mod escaping_writer;
pub use escaping_writer::EscapingWriter;

/// The primary derive macro for enabling RsHtml templating on a struct.
///
/// Apply `#[derive(View)]` to a Rust struct to associate it with an
/// HTML-like template file. This macro processes the template at compile time,
/// generating the necessary Rust code to render it based on the struct's fields
/// and methods.
///
/// By default, the macro attempts to find a template file named after the
/// struct (e.g., `HomePage` struct maps to `views/home.rs.html`).
/// This path can be customized using the `#[view(path = "custom.rs.html", extract)]` attribute
/// on the struct.
///
/// Once derived, an instance of the struct will have a `render(out)` method to produce the HTML output.
pub use rshtml_macro::View;

mod track_views_folder;
/// Instructs Cargo to recompile the crate if any file in the views folder changes.
///
/// This function should be called from a `build.rs` script.
/// It helps ensure that template changes are picked up during development
/// without needing a full manual recompile of the dependent crate.
pub use track_views_folder::track_views_folder;

mod exp;
pub use exp::Exp;
mod view_fn;
pub use view_fn::ViewFn;
mod text_size;
pub use text_size::TextSize;
mod view_iter;
/// Allows iterators to be rendered inside the v macro without the need to call collect.
/// ```rust
/// let card_views = cards
///     .iter()
///     .map(|card| v!(<div class="card">{&card.title}</div>))
///     .view_iter();
///
/// v! {
///     <div>
///         { card_views }
///     </div>
/// }
/// ```
pub use view_iter::ViewIter;

/// Enables writing HTML within Rust and allows for embedding Rust code using the `{}` syntax.
/// The evaluated result is inserted into the output and must implement the `View` or `Display` trait.
/// ```rust
/// v! {
///   <div class="user-info"> {user_info} </div>
///
///   {cards("Card Title", &user.cards)}
/// }
/// ```
pub use rshtml_macro::v;

mod write;
/// To render a `View`, the `render` function requires a type that implements the `Write` trait.
/// This `Write` trait mandates the implementation of `fmt::Write`.
/// Standard library types implementing `fmt::Write` also implement the `Write` trait via `RsHtml`.
pub use write::Write;
mod view;
pub use view::IntoViewIter;
/// The `View` trait makes implementing structs renderable and usable within other views.
pub use view::View;
mod render;
pub use render::Render;
