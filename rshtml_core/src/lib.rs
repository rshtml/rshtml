#![doc(hidden)]

mod compiler;
mod context;
mod diagnostic;
mod extensions;
mod position;
mod rshtml_file;

#[cfg(test)]
mod tests;

pub mod v_macro;

mod debug;
mod extract_file;

pub use compiler::Compiler;
