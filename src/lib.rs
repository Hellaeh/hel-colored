#![feature(test)]
#![feature(fmt_internals)]
// We do implement our own `to_string` method along `fmt::Display`
// TODO custom formatter?
#[allow(clippy::inherent_to_string_shadow_display)]
//
mod ansi;
mod builder;

pub use ansi::ANSIString;
pub use ansi::Colored;
pub use ansi::Styled;

pub use builder::ANSIStringBuilder;
