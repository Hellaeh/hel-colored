//! This crate provides a fast and easy way to color and style string using ANSI escape sequences
//! Repo: https://github.com/Hellaeh/hel-colored
#![feature(test)]
#![feature(const_trait_impl)]
#![feature(formatting_options)]
// #![feature(concat_idents)]
#![warn(missing_docs)]
// We do implement our own `to_string` method along `fmt::Display`
// TODO custom formatter?
#[allow(clippy::inherent_to_string_shadow_display)]
mod ansi;
mod builder;

pub use ansi::ANSIString;
pub use ansi::Color;
pub use ansi::Colored;
pub use ansi::Styled;

pub use builder::ANSIStringBuilder;
